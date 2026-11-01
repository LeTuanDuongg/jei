// Not ported yet: the Java @WrapOperation(INVOKE) on KeyEntry.lambda$new$2 (KeyBindsListEntryMixin.resetJeiModifiers).
// Why it's off: No Horizon function mapped for KeyEntry.lambda$new$2 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for KeyEntry.lambda$new$2: run `match`.
// Note: The call it hooks (KeyMapping.setKey) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyMapping.setKey -> (not mapped yet)
//   KeyEntry.lambda$new$2 -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/KeyBindsListEntryMixin.java:14:
// | @WrapOperation(
// |    method = "lambda$new$2",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/KeyMapping;setKey(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V")
// | )
// | private static void resetJeiModifiers(KeyMapping var0, Key var1, Operation<Void> var2) {
// |    var2.call(new Object[]{var0, var1});
// |    if (var0 instanceof FabricKeyMapping) {
// |       ((FabricKeyMapping)var0).resetModifiersToDefault();
// |    }
// | }
