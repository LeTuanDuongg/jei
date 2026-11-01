// Not ported yet: the Java @Inject(HEAD) on GuiGraphicsExtractor.setTooltipForNextFrameInternal (GuiGraphicsExtractorMixin.jei$setTooltipForNextFrameInternal).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.setTooltipForNextFrameInternal (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GuiGraphicsExtractor.setTooltipForNextFrameInternal: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.componentHoverEffect -> (not mapped yet)
//   GuiGraphicsExtractor.setTooltipForNextFrameInternal -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/GuiGraphicsExtractorMixin.java:19:
// | @Inject(method = "setTooltipForNextFrameInternal", at = @At("HEAD"), cancellable = true)
// | private void jei$setTooltipForNextFrameInternal(
// |    Font var1, List<ClientTooltipComponent> var2, int var3, int var4, ClientTooltipPositioner var5, Identifier var6, boolean var7, CallbackInfo var8
// | ) {
// |    if (!((JeiScreenEvents.AllowDeferredTooltip)JeiScreenEvents.ALLOW_DEFERRED_TOOLTIP.invoker()).allow((GuiGraphicsExtractor)this)) {
// |       var8.cancel();
// |    }
// | }
