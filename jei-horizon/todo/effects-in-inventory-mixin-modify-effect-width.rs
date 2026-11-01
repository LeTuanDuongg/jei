// Not ported yet: the Java @ModifyVariable(STORE) on EffectsInInventory.extractRenderState (EffectsInInventoryMixin.modifyEffectWidth).
// Why it's off: No Horizon function mapped for EffectsInInventory.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for EffectsInInventory.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EffectsInInventory.extractRenderState -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/EffectsInInventoryMixin.java:20:
// | @ModifyVariable(method = "extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V", name = "maxWidth", at = @At("STORE"))
// | public int modifyEffectWidth(int var1) {
// |    return FabricGuiPlugin.getRuntime()
// |          .map(IJeiRuntime::getIngredientListOverlay)
// |          .filter(IngredientListOverlay.class::isInstance)
// |          .map(IngredientListOverlay.class::cast)
// |          .map(var1x -> var1x.shouldRenderCompactInventoryEffects(this.screen))
// |          .orElse(Boolean.FALSE)
// |       ? 32
// |       : var1;
// | }
