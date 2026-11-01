// Not ported yet: the Java @Inject(INVOKE) on AmecsKeyModifiersEarlyInit.onInitialize (AmecsKeyModifiersEarlyInitMixin.beforeSeal).
// Why it's off: No Horizon function mapped for AmecsKeyModifiersEarlyInit.onInitialize (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for AmecsKeyModifiersEarlyInit.onInitialize: run `match`.
// Note: The call it hooks (AmecsKeyModifiers.seal) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AmecsKeyModifiers.seal -> (not mapped yet)
//   AmecsKeyModifiersEarlyInit.onInitialize -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/AmecsKeyModifiersEarlyInitMixin.java:13:
// | @Inject(method = "onInitialize", at = @At(value = "INVOKE", target = "Lde/siphalor/amecs/key_modifiers/api/AmecsKeyModifiers;seal()V"))
// | private void beforeSeal(CallbackInfo var1) {
// |    AmecsSupport.init();
// | }
