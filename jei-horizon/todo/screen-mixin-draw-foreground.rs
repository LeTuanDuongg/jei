// Not ported yet: the Java @Inject(INVOKE) on Screen.extractRenderStateWithTooltipAndSubtitles (ScreenMixin.drawForeground).
// Why it's off: No Horizon function mapped for Screen.extractRenderStateWithTooltipAndSubtitles (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for Screen.extractRenderStateWithTooltipAndSubtitles: run `match`.
// Note: The call it hooks (Screen.extractRenderState) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.nextStratum -> (not mapped yet)
//   GuiGraphicsExtractor.pose -> (not mapped yet)
//   Screen.extractBackground -> horizon_client::game::screens::Screen::blurs_background (suggested, high)
//   Screen.extractRenderState -> (not mapped yet)
//   Screen.extractRenderStateWithTooltipAndSubtitles -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/ScreenMixin.java:29:
// | @Inject(
// |    method = "extractRenderStateWithTooltipAndSubtitles(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/screens/Screen;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V",
// |       ordinal = 0,
// |       shift = Shift.AFTER
// |    )
// | )
// | private void drawForeground(GuiGraphicsExtractor var1, int var2, int var3, float var4, CallbackInfo var5) {
// |    Screen var6;
// |    if (!((var6 = (Screen)this) instanceof AbstractContainerScreen)) {
// |       var1.nextStratum();
// |       runWithIdentityPose(var1, () -> ((JeiScreenEvents.DrawForeground)JeiScreenEvents.DRAW_FOREGROUND.invoker()).drawForeground(var6, var1, var2, var3));
// |    }
// | }
