// Not ported yet: the Java @WrapOperation(INVOKE) on KeyBindsScreen.lambda$addFooter$0 (KeyBindsScreenMixin.resetJeiModifiers).
// Why it's off: No Horizon function mapped for KeyBindsScreen.lambda$addFooter$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for KeyBindsScreen.lambda$addFooter$0: run `match`.
// Note: The call it hooks (KeyMapping.setKey) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   InputConstants.UNKNOWN -> (not mapped yet)
//   InputConstants.getKey -> (not mapped yet)
//   Key.equals -> (not mapped yet)
//   KeyMapping.setKey -> (not mapped yet)
//   OptionsSubScreen.<init> -> (not mapped yet)
//   OptionsSubScreen.keyReleased -> (not mapped yet)
//   KeyBindsList.resetMappingAndUpdateButtons -> (not mapped yet)
//   KeyBindsScreen.keyPressed -> (not mapped yet)
//   KeyBindsScreen.lambda$addFooter$0 -> (not mapped yet)
//   KeyBindsScreen.mouseClicked -> (not mapped yet)
//   KeyEvent.isEscape -> (not mapped yet)
//   KeyEvent.key -> (not mapped yet)
//   KeyEvent.modifiers -> (not mapped yet)
//   MouseButtonEvent.modifiers -> (not mapped yet)
//   Util.getMillis -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/KeyBindsScreenMixin.java:82:
// | @WrapOperation(
// |    method = "lambda$addFooter$0",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/KeyMapping;setKey(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V")
// | )
// | private void resetJeiModifiers(KeyMapping var1, Key var2, Operation<Void> var3) {
// |    var3.call(new Object[]{var1, var2});
// |    if (var1 instanceof ContextAwareKeyMapping) {
// |       ((ContextAwareKeyMapping)var1).resetModifiersToDefault();
// |    }
// | }
