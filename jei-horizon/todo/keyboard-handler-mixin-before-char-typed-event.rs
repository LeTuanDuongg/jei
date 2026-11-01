// Not ported yet: the Java @Inject(HEAD) on KeyboardHandler.charTyped (KeyboardHandlerMixin.beforeCharTypedEvent).
// Why it's off: No Horizon function mapped for KeyboardHandler.charTyped (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for KeyboardHandler.charTyped: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyboardHandler.charTyped -> (not mapped yet)
//   KeyboardHandler.submitPreeditEvent -> (not mapped yet)
//   EditBox.isFocused -> (not mapped yet)
//   Screen.getFocused -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/KeyboardHandlerMixin.java:18:
// | @Inject(method = "charTyped", at = @At("HEAD"), cancellable = true)
// | private void beforeCharTypedEvent(long var1, CharacterEvent var3, CallbackInfo var4) {
// |    beforeCharTypedEventInternal(var1, var3, var4);
// | }
