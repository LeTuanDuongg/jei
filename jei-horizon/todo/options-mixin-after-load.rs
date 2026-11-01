// Not ported yet: the Java @Inject(RETURN) on Options.load (OptionsMixin.afterLoad).
// Why it's off: No Horizon function mapped for Options.load (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Options.load: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyMapping.getName -> (not mapped yet)
//   Options.load -> (not mapped yet)
//   Options.processOptions -> (not mapped yet)
//   FieldAccess.process -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/OptionsMixin.java:27:
// | @Inject(method = "load", at = @At("RETURN"))
// | private void afterLoad(CallbackInfo var1) {
// |    this.jei$loadingOptions = false;
// | }
