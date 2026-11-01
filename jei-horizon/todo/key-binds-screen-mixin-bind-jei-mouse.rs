// Not ported yet: the Java @Inject(HEAD) on KeyBindsScreen.mouseClicked (KeyBindsScreenMixin.bindJeiMouse).
// Why it's off: No Horizon function mapped for KeyBindsScreen.mouseClicked (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for KeyBindsScreen.mouseClicked: run `match`.
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
// Decompiled Java, mezz/jei/fabric/mixin/KeyBindsScreenMixin.java:73:
// | @Inject(method = "mouseClicked", at = @At("HEAD"))
// | private void bindJeiMouse(MouseButtonEvent var1, boolean var2, CallbackInfoReturnable<Boolean> var3) {
// |    KeyMapping var4 = this.selectedKey;
// |    if (this.selectedKey instanceof FabricKeyMapping) {
// |       ((FabricKeyMapping)var4).setModifiers(FabricKeyModifier.fromMask(var1.modifiers()));
// |       this.jei$pendingModifier = null;
// |    }
// | }
