## 2026-10-01 – Patch-0008-Hardwaretest qualifiziert Fall 8B

Der in Block 4.10O präregistrierte adressvergleichende
Kontrolltest wurde am 2026-10-01 genau einmal auf der
Novena ausgeführt.

Die Gültigkeitswerte C0 und C2 waren jeweils Null.

Der Kontrollwert C1 und der Zielwert Q1 waren ebenfalls
jeweils Null.

Damit wird das Ergebnis entsprechend der unveränderten
Präregistrierung als Fall `8B` klassifiziert:

`CONTROL_FAIL_TARGET_FAIL`

Das Ergebnis zeigt, dass die beobachtete fehlgeschlagene
Rücklesung nicht auf die einzelne Target-Adresse
`mem_malloc_brk` beschränkt ist.

Es beweist keinen allgemeinen DDR-Defekt und keinen
konkreten Hardware- oder Softwaremechanismus.

Entscheidungen:

1. Fall 8B wird als gültiges Hardwareergebnis dokumentiert.
2. Die Präregistrierung der Fälle 8A bis 8X bleibt unverändert.
3. Der Patch-0008-Hardwaretest wird nicht wiederholt.
4. Der Originalmitschnitt bleibt unverändert erhalten.
5. Vor weiteren Tests, Builds oder Patches erfolgt ein
   systematischer Quellen- und Ergebnisabgleich.
6. Die bestehende Diagnoseinstrumentierung wird vorerst
   weder entfernt noch als dauerhafte Lösung übernommen.

HARDWARE_TEST_RESULT=8B
PATCH_0008_SECOND_BOOT_ALLOWED=NO
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO
