//! Lokale direkte Kindprozess-Lebensdauer, kein Erwerber und keine Isolation.
//! Der Aufrufer startet ausschließlich separat autorisierte Prozesse und muss
//! Pipes, Prozessbaum, cgroups, Quoten und Credentials extern kontrollieren.
//! Insbesondere darf dieser Teilvertrag noch keinen SSH-Erwerber freigeben.
use crate::{BridgeError, BridgeResult};
use std::process::Child;
use std::time::{Duration, Instant};

/// Übernimmt ein bereits gestartetes Kind. Kein Launcher, keine Shell-API.
/// Drop beendet und reap't das direkte Kind. Nachkommen sind NICHT erfasst.
pub struct LocalChildGuard {
    child: Child,
    finished: bool,
}
impl LocalChildGuard {
    pub fn new(child: Child) -> Self {
        Self {
            child,
            finished: false,
        }
    }

    /// Einmalig; abgelaufene Deadline wird vor jeder Exitbeobachtung geprüft.
    /// Erfolg bedeutet nur Exit 0, niemals geprüfte Erwerbs-/Objektevidenz.
    /// kill/wait sind OS-Aufrufe ohne behauptete präemptive Kerneldeadline.
    pub fn wait_until(&mut self, deadline: Instant) -> BridgeResult<()> {
        if self.finished {
            return Err(BridgeError("LOCAL_CHILD_ALREADY_FINISHED"));
        }
        let result = loop {
            let now = Instant::now();
            if now >= deadline {
                break Err(BridgeError("LOCAL_CHILD_TIMEOUT"));
            }
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.finished = true;
                    if Instant::now() >= deadline {
                        break Err(BridgeError("LOCAL_CHILD_TIMEOUT"));
                    }
                    break if status.success() {
                        Ok(())
                    } else {
                        Err(BridgeError("LOCAL_CHILD_FAILED"))
                    };
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(5).min(deadline - now)),
                Err(_) => break Err(BridgeError("LOCAL_CHILD_WAIT")),
            }
        };
        if !self.finished {
            // Auch bei Timeout oder Beobachtungsfehler kein verwaistes direktes Kind.
            self.stop()?;
        }
        result
    }
    fn stop(&mut self) -> BridgeResult<()> {
        // Ein Exit zwischen try_wait und kill ist erlaubt; wait entscheidet,
        // ob das Kind tatsächlich reap't wurde, nicht der kill-Exitcode allein.
        let _ = self.child.kill();
        self.child
            .wait()
            .map_err(|_| BridgeError("LOCAL_CHILD_REAP"))?;
        self.finished = true;
        Ok(())
    }
}
impl Drop for LocalChildGuard {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.stop();
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};
    fn child(executable: &str, args: &[&str]) -> LocalChildGuard {
        LocalChildGuard::new(
            Command::new(executable)
                .args(args)
                .env_clear()
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap(),
        )
    }
    #[test]
    fn local_exit_is_one_shot_and_nonzero_is_not_success() {
        for (exe, expected) in [
            ("/bin/true", Ok(())),
            ("/bin/false", Err(BridgeError("LOCAL_CHILD_FAILED"))),
        ] {
            let mut guard = child(exe, &[]);
            assert_eq!(
                guard.wait_until(Instant::now() + Duration::from_secs(5)),
                expected
            );
            assert!(guard.child.try_wait().unwrap().is_some());
            assert_eq!(
                guard.wait_until(Instant::now() + Duration::from_secs(5)),
                Err(BridgeError("LOCAL_CHILD_ALREADY_FINISHED"))
            );
        }
    }
    #[test]
    fn expired_deadline_kills_and_reaps_without_retry() {
        let mut guard = child("/bin/sleep", &["30"]);
        assert_eq!(
            guard.wait_until(Instant::now()),
            Err(BridgeError("LOCAL_CHILD_TIMEOUT"))
        );
        assert!(guard.finished);
        assert!(guard.child.try_wait().unwrap().is_some());
        assert_eq!(
            guard.wait_until(Instant::now() + Duration::from_secs(5)),
            Err(BridgeError("LOCAL_CHILD_ALREADY_FINISHED"))
        );
    }
    #[test]
    fn drop_reaps_direct_child_observed_independently_in_procfs() {
        let guard = child("/bin/sleep", &["30"]);
        let id = guard.child.id().to_string();
        let children = || std::fs::read_to_string("/proc/thread-self/children").unwrap();
        assert!(children().split_whitespace().any(|pid| pid == id));
        drop(guard);
        assert!(!children().split_whitespace().any(|pid| pid == id));
    }
}
