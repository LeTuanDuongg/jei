// Not ported yet: the Java @Inject(RETURN) on ClientPacketListener.handleUpdateRecipes (ClientPacketListenerRecipeUpdateMixin.handleUpdateRecipes).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleUpdateRecipes (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ClientPacketListener.handleUpdateRecipes: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.handleUpdateRecipes -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/ClientPacketListenerRecipeUpdateMixin.java:13:
// | @Inject(method = "handleUpdateRecipes", at = @At("RETURN"))
// | public void handleUpdateRecipes(ClientboundUpdateRecipesPacket var1, CallbackInfo var2) {
// |    ((Runnable)JeiLifecycleEvents.AFTER_RECIPES_UPDATED.invoker()).run();
// | }
