// Not ported yet: the Java @Inject(HEAD) on GuiGraphicsExtractor.componentHoverEffect (GuiGraphicsExtractorMixin.jei$componentHoverEffect).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.componentHoverEffect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GuiGraphicsExtractor.componentHoverEffect: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.componentHoverEffect -> (not mapped yet)
//   GuiGraphicsExtractor.setTooltipForNextFrameInternal -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/GuiGraphicsExtractorMixin.java:28:
// | @Inject(method = "componentHoverEffect", at = @At("HEAD"), cancellable = true)
// | private void jei$componentHoverEffect(Font var1, Style var2, int var3, int var4, CallbackInfo var5) {
// |    if (ChatIngredientTooltip.setTooltipForHoveredText((GuiGraphicsExtractor)this, var2, var3, var4)) {
// |       var5.cancel();
// |    }
// | }
