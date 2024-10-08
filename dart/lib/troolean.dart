// ignore_for_file: constant_identifier_names
// Basic Klene / Priest three-valued logic

enum Troolean {
  False,
  True,
  Unknown;

  bool isTrue() {
    return this == Troolean.True;
  }

  bool isFalse() {
    return this == Troolean.False;
  }

  bool isUnknown() {
    return this == Troolean.Unknown;
  }

  bool isKnown() {
    return this != Troolean.Unknown;
  }

  Troolean not() {
    switch (this) {
      case Troolean.True:
        return Troolean.False;
      case Troolean.False:
        return Troolean.True;
      default:
        return Troolean.Unknown;
    }
  }

  Troolean and(Troolean other) {
    switch (this) {
      case Troolean.True:
        return other;
      case Troolean.False:
        return Troolean.False;
      default:
        return Troolean.Unknown;
    }
  }

  Troolean or(Troolean other) {
    switch (this) {
      case Troolean.False:
        return other;
      case Troolean.True:
        return Troolean.True;
      default:
        return Troolean.Unknown;
    }
  }

  Troolean xor(Troolean other) {
    switch (this) {
      case Troolean.True:
        return other.not();
      case Troolean.False:
        return other;
      default:
        return Troolean.Unknown;
    }
  }
}

const Troolean TTRUE = Troolean.True;
const Troolean TFALSE = Troolean.False;
const Troolean TUNKNOWN = Troolean.Unknown;