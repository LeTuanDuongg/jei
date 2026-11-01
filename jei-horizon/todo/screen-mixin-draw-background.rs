// Not ported yet: the Java @Inject(INVOKE) on Screen.extractRenderStateWithTooltipAndSubtitles (ScreenMixin.drawBackground).
// Why it's off: No Horizon function mapped for Screen.extractRenderStateWithTooltipAndSubtitles (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for Screen.extractRenderStateWithTooltipAndSubtitles: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.nextStratum -> (not mapped yet)
//   GuiGraphicsExtractor.pose -> (not mapped yet)
//   Screen.extractBackground -> horizon_client::game::screens::Screen::blurs_background (suggested, high)
//   Screen.extractRenderState -> (not mapped yet)
//   Screen.extractRenderStateWithTooltipAndSubtitles -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/ScreenMixin.java:16:
// | @Inject(
// |    method = "extractRenderStateWithTooltipAndSubtitles(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V",
// |       ordinal = 0,
// |       shift = Shift.AFTER
// |    )
// | )
// | private void drawBackground(GuiGraphicsExtractor var1, int var2, int var3, float var4, CallbackInfo var5) {
// |    runWithIdentityPose(var1, () -> ((JeiScreenEvents.DrawBackground)JeiScreenEvents.DRAW_BACKGROUND.invoker()).drawBackground(var0, var1));
// | }
