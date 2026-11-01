// Not ported yet: the Java @Inject(HEAD) on KeyBindsScreen.keyPressed (KeyBindsScreenMixin.bindJeiKeyboard).
// Why it's off: No Horizon function mapped for KeyBindsScreen.keyPressed (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for KeyBindsScreen.keyPressed: run `match`.
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
// Decompiled Java, mezz/jei/fabric/mixin/KeyBindsScreenMixin.java:44:
// | @Inject(method = "keyPressed", at = @At("HEAD"), cancellable = true)
// | private void bindJeiKeyboard(KeyEvent var1, CallbackInfoReturnable<Boolean> var2) {
// |    FabricKeyMapping var3 = this.selectedKey;
// |    if (this.selectedKey instanceof FabricKeyMapping) {
// |       var3 = var3;
// |       Key var4 = InputConstants.getKey(var1);
// |       if (var1.isEscape()) {
// |          var4 = InputConstants.UNKNOWN;
// |       }
// |
// |       FabricKeyModifier var5;
// |       FabricKeyModifier var10000 = var5 = FabricKeyModifier.forKey(var1.key());
// |       EnumSet var6 = FabricKeyModifier.fromMask(var1.modifiers());
// |       if (var10000 != null) {
// |          var6.remove(var5);
// |       }
// |
// |       var3.setKeyAndModifiers(var4, var6);
// |       if (var5 != null) {
// |          this.jei$pendingModifier = var4;
// |          this.keyBindsList.resetMappingAndUpdateButtons();
// |       } else {
// |          this.jei$finishBinding();
// |       }
// |
// |       var2.setReturnValue(Boolean.TRUE);
// |    }
// | }
