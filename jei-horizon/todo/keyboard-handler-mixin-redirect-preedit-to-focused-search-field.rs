// Not ported yet: the Java @ModifyVariable(HEAD) on KeyboardHandler.submitPreeditEvent (KeyboardHandlerMixin.redirectPreeditToFocusedSearchField).
// Why it's off: No Horizon function mapped for KeyboardHandler.submitPreeditEvent (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for KeyboardHandler.submitPreeditEvent: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyboardHandler.charTyped -> (not mapped yet)
//   KeyboardHandler.submitPreeditEvent -> (not mapped yet)
//   EditBox.isFocused -> (not mapped yet)
//   Screen.getFocused -> (not mapped yet)
//
// Decompiled Java, mezz/jei/fabric/mixin/KeyboardHandlerMixin.java:28:
// | @ModifyVariable(method = "submitPreeditEvent", at = @At("HEAD"), argsOnly = true)
// | private static GuiEventListener redirectPreeditToFocusedSearchField(GuiEventListener var0) {
// |    GuiEventListener var1;
// |    GuiTextFieldFilter var2;
// |    return (GuiEventListener)(var0 instanceof Screen
// |          && (var1 = ((Screen)var0).getFocused()) instanceof GuiTextFieldFilter
// |          && (var2 = (GuiTextFieldFilter)var1).isFocused()
// |       ? var2
// |       : var0);
// | }
