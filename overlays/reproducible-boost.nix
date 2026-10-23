final: prev:

{
  boost =
    if prev.stdenv.hostPlatform != prev.stdenv.buildPlatform then
      prev.boost.override (old: {
        # Boost 1.89.0's Stacktrace Jamfile handles "off" before probing
        # addr2line. nixpkgs passes extraB2Args to both b2 invocations.
        extraB2Args = (old.extraB2Args or [ ]) ++ [
          "boost.stacktrace.addr2line=off"
        ];
      })
    else
      prev.boost;
}
