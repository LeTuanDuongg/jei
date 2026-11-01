// Not ported yet: the Java @Inject(INVOKE) on Minecraft.<init> (MinecraftMixin.beforeInitialResourceReload).
// Why it's off: No Horizon function mapped for Minecraft.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for Minecraft.<init>: run `match`.
// Note: The call it hooks (ResourceLoadStateTracker.startReload) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.<init> -> (not mapped yet)
//   Minecraft.updateLevelInEngines -> (not mapped yet)
//   ResourceLoadStateTracker.startReload -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/MinecraftMixin.java:25:
// | @Inject(
// |    method = "<init>(Lnet/minecraft/client/main/GameConfig;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/ResourceLoadStateTracker;startReload(Lnet/minecraft/client/ResourceLoadStateTracker$ReloadReason;Ljava/util/List;)V",
// |       ordinal = 0
// |    )
// | )
// | public void beforeInitialResourceReload(GameConfig var1, CallbackInfo var2) {
// |    JeiLifecycleEvents.RegisterResourceReloadListener var10000 = (JeiLifecycleEvents.RegisterResourceReloadListener)JeiLifecycleEvents.REGISTER_RESOURCE_RELOAD_LISTENER
// |       .invoker();
// |    ReloadableResourceManager var3 = this.resourceManager;
// |    var10000.registerResourceReloadListener(var3, this.textureManager);
// | }
