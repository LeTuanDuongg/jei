// Not ported yet: the Java @Inject(HEAD) on Options.load (OptionsMixin.beforeLoad).
// Why it's off: No Horizon function mapped for Options.load (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Options.load: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyMapping.getName -> (not mapped yet)
//   Options.load -> (not mapped yet)
//   Options.processOptions -> (not mapped yet)
//   FieldAccess.process -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/OptionsMixin.java:22:
// | @Inject(method = "load", at = @At("HEAD"))
// | private void beforeLoad(CallbackInfo var1) {
// |    this.jei$loadingOptions = true;
// | }
