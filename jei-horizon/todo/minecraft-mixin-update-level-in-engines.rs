// Not ported yet: the Java @Inject(HEAD) on Minecraft.updateLevelInEngines (MinecraftMixin.updateLevelInEngines).
// Why it's off: No Horizon function mapped for Minecraft.updateLevelInEngines (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.updateLevelInEngines: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.<init> -> (not mapped yet)
//   Minecraft.updateLevelInEngines -> (not mapped yet)
//   ResourceLoadStateTracker.startReload -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/MinecraftMixin.java:40:
// | @Inject(method = "updateLevelInEngines(Lnet/minecraft/client/multiplayer/ClientLevel;Z)V", at = @At("HEAD"))
// | public void updateLevelInEngines(ClientLevel var1, boolean var2, CallbackInfo var3) {
// |    if (var1 == null) {
// |       ((Runnable)JeiLifecycleEvents.GAME_STOP.invoker()).run();
// |    }
// | }
