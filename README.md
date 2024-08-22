Arbitrary precision arthmatic from scratch in rust

Part of a project to complete CAScubirb

Structs:
* BigFloat: Arbitrary precision floats, using vectors.
* Medium: Medium sized, high precision math with BigFloat and a power. Maximum is technically unlimited, but it will be clamped to 10^(2^64-1).
* Large: Special number type supporting up to 10^^^^(2^64-1)
* Omega num: Supports up to 10{1024}10

Theory of operation:
Arbitrary.rs: Bog-standard vector based number representations.

Medium: Exponent and power. Exponent is a BigFloat, and the power is an integer. 

Large: A modified vector, where all numbers are a\*10^b\*10^^c\*10^^^d\*10^^^^e. This is then stored as exponent = \[b,c,d,e\] and mantissia = a.

OmegaNum: Same as large, but this time the length of the vector maximum is changed to 1024, or even more depending on need.
