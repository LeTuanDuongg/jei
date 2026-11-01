// Not ported yet: the Java @Inject(TAIL) on Options.processOptions (OptionsMixin.processJeiModifiers).
// Why it's off: No Horizon function mapped for Options.processOptions (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Options.processOptions: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyMapping.getName -> (not mapped yet)
//   Options.load -> (not mapped yet)
//   Options.processOptions -> (not mapped yet)
//   FieldAccess.process -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/OptionsMixin.java:32:
// | @Inject(method = "processOptions", at = @At("TAIL"))
// | private void processJeiModifiers(FieldAccess var1, CallbackInfo var2) {
// |    KeyMapping[] var8 = this.keyMappings;
// |    int var3 = this.keyMappings.length;
// |
// |    for (int var4 = 0; var4 < var3; var4++) {
// |       KeyMapping var5;
// |       if ((var5 = var8[var4]) instanceof FabricKeyMapping) {
// |          FabricKeyMapping var6;
// |          int var7 = FabricKeyModifier.toMask((var6 = (FabricKeyMapping)var5).getModifiers());
// |          if (this.jei$loadingOptions && !var1.process("key_" + var5.getName(), "").isEmpty()) {
// |             var7 = 0;
// |          }
// |
// |          var6.setModifiers(FabricKeyModifier.fromMask(var1.process("jei_modifiers_" + var5.getName(), var7)));
// |       }
// |    }
// | }
