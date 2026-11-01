// Not ported yet: the Java @Inject(TAIL) on KeyboardHandler.charTyped (KeyboardHandlerMixin.afterCharTypedEvent).
// Why it's off: No Horizon function mapped for KeyboardHandler.charTyped (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for KeyboardHandler.charTyped: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyboardHandler.charTyped -> (not mapped yet)
//   KeyboardHandler.submitPreeditEvent -> (not mapped yet)
//   EditBox.isFocused -> (not mapped yet)
//   Screen.getFocused -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/KeyboardHandlerMixin.java:23:
// | @Inject(method = "charTyped", at = @At("TAIL"), cancellable = true)
// | private void afterCharTypedEvent(long var1, CharacterEvent var3, CallbackInfo var4) {
// |    afterCharTypedEventInternal(var1, var3, var4);
// | }
