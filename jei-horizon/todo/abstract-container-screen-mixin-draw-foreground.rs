// Not ported yet: the Java @Inject(HEAD) on AbstractContainerScreen.extractCarriedItem (AbstractContainerScreenMixin.drawForeground).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.extractCarriedItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AbstractContainerScreen.extractCarriedItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.nextStratum -> (not mapped yet)
//   GuiGraphicsExtractor.pose -> (not mapped yet)
//   AbstractContainerScreen.extractCarriedItem -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/AbstractContainerScreenMixin.java:14:
// | @Inject(method = "extractCarriedItem(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V", at = @At("HEAD"))
// | private void drawForeground(GuiGraphicsExtractor var1, int var2, int var3, CallbackInfo var4) {
// |    AbstractContainerScreen var10001 = (AbstractContainerScreen)this;
// |    var1.nextStratum();
// |    runWithIdentityPose(var1, () -> ((JeiScreenEvents.DrawForeground)JeiScreenEvents.DRAW_FOREGROUND.invoker()).drawForeground(var10001, var1, var2, var3));
// | }
