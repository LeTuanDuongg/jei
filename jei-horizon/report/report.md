# Horizon port report: Just Enough Items

`jei` 31.17.0.108 · jei-26.3-fabric-31.17.0.108.jar · 2696 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 3 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 2 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 20 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 8 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 13 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: mezz.jei.fabric.JustEnoughItems
- `client`: mezz.jei.fabric.JustEnoughItemsClient
- `jei_mod_plugin`: mezz.jei.library.plugins.vanilla.VanillaPlugin, mezz.jei.library.plugins.jei.JeiInternalPlugin, mezz.jei.fabric.plugins.fabric.FabricGuiPlugin, mezz.jei.gui.plugins.JeiGuiPlugin
- `mezz_config_gui_plugin`: mezz.jei.fabric.input.FabricKeyMappingConfigGuiPlugin

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ClientLifecycleEvents.CLIENT_STARTED | `JustEnoughItemsClient` | – | needs-translation | Start/stop: usually the first tick hook call, or nothing. |
| ClientLifecycleEvents.CLIENT_STOPPING | `JustEnoughItemsClient` | – | needs-translation | Start/stop: usually the first tick hook call, or nothing. |
| ClientCommandRegistrationCallback.EVENT | `JeiInternalShowCommand` | hook `command` | auto | Brigadier argument trees need translating to Horizon's command hook arguments. |
| KeyMappingHelper.getBoundKeyOf | `AmecsKeyMapping` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ClientPlayNetworking.registerGlobalReceiver | `ClientNetworkHandler` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.send | `ConnectionToClient` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.canSend | `ConnectionToServer` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.send | `ConnectionToServer` | – | unsupported | Custom packets aren't supported. |
| PayloadTypeRegistry.clientboundPlay | `ServerNetworkHandler` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `ServerNetworkHandler` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.serverboundPlay | `ServerNetworkHandler` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `ServerNetworkHandler` | – | unsupported | Custom packets aren't supported. |
| KeyMappingHelper.registerKeyMapping | `ClientLifecycleHandler` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ScreenEvents.AFTER_INIT | `ClientLifecycleHandler` | – | unsupported | No screen hook in Horizon yet. |
| ScreenEvents.BEFORE_INIT | `EventRegistration` | – | unsupported | No screen hook in Horizon yet. |
| ScreenEvents.afterTick | `EventRegistration` | – | unsupported | No screen hook in Horizon yet. |
| ScreenEvents.remove | `EventRegistration` | – | unsupported | No screen hook in Horizon yet. |
| ScreenEvents.beforeExtract | `EventRegistration` | – | unsupported | No screen hook in Horizon yet. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on AbstractContainerScreen.extractCarriedItem | `AbstractContainerScreenMixin.drawForeground` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.extractCarriedItem: run `match`. |
| @Inject(INVOKE) on AmecsKeyModifiersEarlyInit.onInitialize at Lde/siphalor/amecs/key_modifiers/api/AmecsKeyModifiers;seal()V | `AmecsKeyModifiersEarlyInitMixin.beforeSeal` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for AmecsKeyModifiersEarlyInit.onInitialize: run `match`. The call it hooks (AmecsKeyModifiers.seal) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on ClientPacketListener.handleUpdateRecipes | `ClientPacketListenerRecipeUpdateMixin.handleUpdateRecipes` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ClientPacketListener.handleUpdateRecipes: run `match`. |
| @ModifyVariable(STORE) on EffectsInInventory.extractRenderState | `EffectsInInventoryMixin.modifyEffectWidth` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for EffectsInInventory.extractRenderState: run `match`. |
| @Inject(HEAD) on GuiGraphicsExtractor.setTooltipForNextFrameInternal | `GuiGraphicsExtractorMixin.jei$setTooltipForNextFrameInternal` | HEAD | needs-mapping | No Horizon function known for GuiGraphicsExtractor.setTooltipForNextFrameInternal: run `match`. |
| @Inject(HEAD) on GuiGraphicsExtractor.componentHoverEffect | `GuiGraphicsExtractorMixin.jei$componentHoverEffect` | HEAD | needs-mapping | No Horizon function known for GuiGraphicsExtractor.componentHoverEffect: run `match`. |
| @WrapOperation(INVOKE) on KeyEntry.lambda$new$2 at Lnet/minecraft/client/KeyMapping;setKey(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V | `KeyBindsListEntryMixin.resetJeiModifiers` | WRAP_OPERATION | needs-mapping | No Horizon function known for KeyEntry.lambda$new$2: run `match`. The call it hooks (KeyMapping.setKey) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on KeyBindsScreen.keyPressed | `KeyBindsScreenMixin.bindJeiKeyboard` | HEAD | needs-mapping | No Horizon function known for KeyBindsScreen.keyPressed: run `match`. |
| @Inject(HEAD) on KeyBindsScreen.mouseClicked | `KeyBindsScreenMixin.bindJeiMouse` | HEAD | needs-mapping | No Horizon function known for KeyBindsScreen.mouseClicked: run `match`. |
| @WrapOperation(INVOKE) on KeyBindsScreen.lambda$addFooter$0 at Lnet/minecraft/client/KeyMapping;setKey(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V | `KeyBindsScreenMixin.resetJeiModifiers` | WRAP_OPERATION | needs-mapping | No Horizon function known for KeyBindsScreen.lambda$addFooter$0: run `match`. The call it hooks (KeyMapping.setKey) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on KeyboardHandler.charTyped | `KeyboardHandlerMixin.beforeCharTypedEvent` | HEAD | needs-mapping | No Horizon function known for KeyboardHandler.charTyped: run `match`. |
| @Inject(TAIL) on KeyboardHandler.charTyped | `KeyboardHandlerMixin.afterCharTypedEvent` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for KeyboardHandler.charTyped: run `match`. |
| @ModifyVariable(HEAD) on KeyboardHandler.submitPreeditEvent | `KeyboardHandlerMixin.redirectPreeditToFocusedSearchField` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for KeyboardHandler.submitPreeditEvent: run `match`. |
| @Inject(INVOKE) on Minecraft.<init> at Lnet/minecraft/client/ResourceLoadStateTracker;startReload(Lnet/minecraft/client/ResourceLoadStateTracker$ReloadReason;Ljava/util/List;)V | `MinecraftMixin.beforeInitialResourceReload` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Minecraft.<init>: run `match`. The call it hooks (ResourceLoadStateTracker.startReload) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on Minecraft.updateLevelInEngines | `MinecraftMixin.updateLevelInEngines` | HEAD | needs-mapping | No Horizon function known for Minecraft.updateLevelInEngines: run `match`. |
| @Inject(HEAD) on Options.load | `OptionsMixin.beforeLoad` | HEAD | needs-mapping | No Horizon function known for Options.load: run `match`. |
| @Inject(RETURN) on Options.load | `OptionsMixin.afterLoad` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Options.load: run `match`. |
| @Inject(TAIL) on Options.processOptions | `OptionsMixin.processJeiModifiers` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Options.processOptions: run `match`. |
| @Inject(INVOKE) on Screen.extractRenderStateWithTooltipAndSubtitles at Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V | `ScreenMixin.drawBackground` | AFTER_CALL | needs-mapping | No Horizon function known for Screen.extractRenderStateWithTooltipAndSubtitles: run `match`. |
| @Inject(INVOKE) on Screen.extractRenderStateWithTooltipAndSubtitles at Lnet/minecraft/client/gui/screens/Screen;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V | `ScreenMixin.drawForeground` | AFTER_CALL | needs-mapping | No Horizon function known for Screen.extractRenderStateWithTooltipAndSubtitles: run `match`. The call it hooks (Screen.extractRenderState) isn't mapped to a Rust function yet. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends KeyMapping | `FabricKeyMapping` | needs-translation | No mapping for this superclass yet. |
| extends Button | `InternalIconButton` | needs-translation | No mapping for this superclass yet. |
| extends EditBox | `GuiTextFieldFilter` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `RecipesGui` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends SmeltingRecipe | `JeiSmeltingRecipe` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `jei.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `JustEnoughItems` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 104 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

41 of 1002 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/platform/InputConstants$Key#getDisplayName` | horizon_client_core::key_display::display_name (suggested) |
| `net/minecraft/ChatFormatting#stripFormatting` | horizon_client_input::clipboard::strip_formatting (suggested) |
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyboardHandler#setClipboard` | horizon_client::game::debug_keys::Game<'_>::set_clipboard (suggested) |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#hasControlDown` | horizon_client::game::Game<'_>::has_control_down (suggested) |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/StringSplitter#splitLines` | horizon_client_render::font::Font::split_lines (suggested) |
| `net/minecraft/client/gui/Font#plainSubstrByWidth` | horizon_client_render::font::Font::fit (suggested) |
| `net/minecraft/client/gui/Font#split` | horizon_client_render::font::Font::split_formatted (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#requestCursor` | horizon_client_render::widgets::request_cursor (suggested) |
| `net/minecraft/client/gui/components/EditBox#keyPressed` | horizon_client_render::ui::EditBox::shortcut (suggested) |
| `net/minecraft/client/gui/screens/Screen#extractBackground` | horizon_client::game::screens::Screen::blurs_background (suggested) |
| `net/minecraft/client/multiplayer/chat/ChatAbilities#hasAnyRestrictions` | horizon_client_render::chat::ChatAbilities::any_restriction (suggested) |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#forUI` | horizon_client_render::sound_engine::SoundInstance::for_ui (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/Connection#isMemoryConnection` | horizon_protocol::connection::Connection::is_memory (suggested) |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#list` | horizon_protocol::item::skip::list (suggested) |
| `net/minecraft/server/level/ServerPlayer#getDisplayName` | horizon_client::lan::hub::display_name (suggested) |
| `net/minecraft/util/ARGB#white` | horizon_client_render::hud_layout::white (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/StringUtil#isAllowedChatCharacter` | horizon_client_input::clipboard::is_allowed_chat_character (suggested) |
| `net/minecraft/world/effect/MobEffectUtil#formatDuration` | horizon_client_render::brewing::format_duration (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Player#canUseGameMasterBlocks` | horizon_client::game::command_blocks::Game<'_>::can_use_game_master_blocks (suggested) |
| `net/minecraft/world/inventory/Slot#safeInsert` | horizon_client::menu::Menu::safe_insert (suggested) |
| `net/minecraft/world/item/ItemStack#enchant` | horizon_items::item_data::ItemStack::enchant (suggested) |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#matches` | horizon_client_render::vault::matches (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#shrink` | horizon_client::lan::advancements::shrink (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 4 |
| `com/mojang/blaze3d/platform/InputConstants#getKey` | method | 4 |
| `com/mojang/blaze3d/platform/InputConstants#isKeyDown` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Key#equals` | method | 7 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getName` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getType` | method | 6 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 10 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 5 |
| `com/mojang/blaze3d/platform/InputConstants$Type#MOUSE` | field | 12 |
| `com/mojang/blaze3d/platform/InputConstants$Type#getOrCreate` | method | 5 |
| `com/mojang/blaze3d/platform/InputConstants$Type#ordinal` | method | 4 |
| `com/mojang/blaze3d/platform/InputConstants$Type#values` | method | 2 |
| `com/mojang/blaze3d/platform/MacosUtil#IS_MACOS` | field | 1 |
| `com/mojang/blaze3d/platform/NativeImage#close` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#format` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#getHeight` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#getPixel` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#getWidth` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#read` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage$Format#RGBA` | field | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#handle` | method | 1 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#ARROW` | field | 2 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#NOT_ALLOWED` | field | 1 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#POINTING_HAND` | field | 3 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#RESIZE_ALL` | field | 1 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#RESIZE_EW` | field | 1 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#RESIZE_NS` | field | 3 |
| `de/siphalor/amecs/key_modifiers/api/AmecsKeyModifiers#seal` | method | 1 |
| `de/siphalor/amecs/key_modifiers/impl/AmecsKeyModifiersEarlyInit#onInitialize` | method | 1 |
| `net/minecraft/ChatFormatting#AQUA` | field | 1 |
| `net/minecraft/ChatFormatting#BLUE` | field | 2 |
| `net/minecraft/ChatFormatting#BOLD` | field | 3 |
| `net/minecraft/ChatFormatting#DARK_BLUE` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 2 |
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_RED` | field | 1 |
| `net/minecraft/ChatFormatting#GOLD` | field | 4 |
| `net/minecraft/ChatFormatting#GRAY` | field | 10 |
| `net/minecraft/ChatFormatting#ITALIC` | field | 7 |
| `net/minecraft/ChatFormatting#OBFUSCATED` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 7 |
| `net/minecraft/ChatFormatting#RESET` | field | 2 |
| `net/minecraft/ChatFormatting#STRIKETHROUGH` | field | 1 |
| `net/minecraft/ChatFormatting#UNDERLINE` | field | 1 |
| `net/minecraft/ChatFormatting#WHITE` | field | 1 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 1 |
| `net/minecraft/ChatFormatting#getByCode` | method | 1 |
| `net/minecraft/ChatFormatting#name` | method | 1 |
| `net/minecraft/ChatFormatting#values` | method | 3 |
| `net/minecraft/CrashReport#addCategory` | method | 2 |
| `net/minecraft/CrashReport#forThrowable` | method | 1 |
| `net/minecraft/CrashReportCategory#<init>` | method | 1 |
| `net/minecraft/CrashReportCategory#getDetails` | method | 1 |
| `net/minecraft/CrashReportCategory#setDetail` | method | 2 |
| `net/minecraft/ReportedException#<init>` | method | 2 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 2 |
| `net/minecraft/client/KeyMapping#getCategory` | method | 1 |
| `net/minecraft/client/KeyMapping#getDefaultKey` | method | 2 |
| `net/minecraft/client/KeyMapping#getName` | method | 1 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 1 |
| `net/minecraft/client/KeyMapping#isUnbound` | method | 2 |
| `net/minecraft/client/KeyMapping#matches` | method | 1 |
| `net/minecraft/client/KeyMapping#matchesMouse` | method | 1 |
| `net/minecraft/client/KeyMapping#resetMapping` | method | 1 |
| `net/minecraft/client/KeyMapping#setKey` | method | 3 |
| `net/minecraft/client/KeyMapping$Category#<init>` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#id` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#label` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/KeyboardHandler#charTyped` | method | 2 |
| `net/minecraft/client/KeyboardHandler#submitPreeditEvent` | method | 1 |
| `net/minecraft/client/Minecraft#<init>` | method | 1 |
| `net/minecraft/client/Minecraft#gameMode` | field | 1 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 2 |
| `net/minecraft/client/Minecraft#getBlockColors` | method | 1 |
| `net/minecraft/client/Minecraft#getConnection` | method | 3 |
| `net/minecraft/client/Minecraft#getCurrentServer` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getLanguageManager` | method | 1 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 2 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 1 |
| `net/minecraft/client/Minecraft#getSingleplayerServer` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 3 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 1 |
| `net/minecraft/client/Minecraft#getUser` | method | 2 |
| `net/minecraft/client/Minecraft#getWindow` | method | 5 |
| `net/minecraft/client/Minecraft#gui` | field | 22 |
| `net/minecraft/client/Minecraft#hasAltDown` | method | 1 |
| `net/minecraft/client/Minecraft#hasShiftDown` | method | 4 |
| `net/minecraft/client/Minecraft#isSameThread` | method | 2 |
| `net/minecraft/client/Minecraft#keyboardHandler` | field | 1 |
| `net/minecraft/client/Minecraft#level` | field | 12 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 2 |
| `net/minecraft/client/Minecraft#options` | field | 7 |
| `net/minecraft/client/Minecraft#schedule` | method | 1 |
| `net/minecraft/client/Minecraft#setScreenAndShow` | method | 1 |
| `net/minecraft/client/Minecraft#updateLevelInEngines` | method | 1 |
| `net/minecraft/client/MouseHandler#isLeftPressed` | method | 1 |
| `net/minecraft/client/MouseHandler#isMiddlePressed` | method | 1 |
| `net/minecraft/client/MouseHandler#isRightPressed` | method | 1 |
| `net/minecraft/client/MouseHandler#xpos` | method | 1 |
| `net/minecraft/client/MouseHandler#ypos` | method | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 1 |
| `net/minecraft/client/Options#advancedItemTooltips` | field | 3 |
| `net/minecraft/client/Options#keyHotbarSlots` | field | 1 |
| `net/minecraft/client/Options#keyInventory` | field | 1 |
| `net/minecraft/client/Options#keyPickItem` | field | 1 |
| `net/minecraft/client/Options#load` | method | 2 |
| `net/minecraft/client/Options#operatorItemsTab` | method | 1 |
| `net/minecraft/client/Options#processOptions` | method | 1 |
| `net/minecraft/client/Options$FieldAccess#process` | method | 2 |
| `net/minecraft/client/ResourceLoadStateTracker#startReload` | method | 1 |
| `net/minecraft/client/User#getProfileId` | method | 2 |
| `net/minecraft/client/color/block/BlockColors#getTintSource` | method | 1 |
| `net/minecraft/client/color/block/BlockTintSource#color` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector#defaultScrollingHelper` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector#isPointInRectangle` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$ClickableStyleFinder#<init>` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$ClickableStyleFinder#result` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#<init>` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#opacity` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#pose` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#scissor` | method | 1 |
| `net/minecraft/client/gui/Font#getSplitter` | method | 1 |
| `net/minecraft/client/gui/Font#substrByWidth` | method | 1 |
| `net/minecraft/client/gui/Font$PreparedText#visit` | method | 1 |
| `net/minecraft/client/gui/Gui#chatListener` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/Gui#overlay` | method | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 19 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 2 |
| `net/minecraft/client/gui/Gui#toastManager` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitNineSlicedSprite` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitTiledSprite` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#componentHoverEffect` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#disableScissor` | method | 8 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#enableScissor` | method | 8 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fakeItem` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 15 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fillGradient` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiHeight` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiWidth` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#nextStratum` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 17 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrameInternal` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 11 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#tooltip` | method | 1 |
| `net/minecraft/client/gui/Hud#getChat` | method | 1 |
| `net/minecraft/client/gui/Hud#getGuiTicks` | method | 1 |
| `net/minecraft/client/gui/TextAlignment#calculateLeft` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#isFocused` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/components/Button#<init>` | method | 1 |
| `net/minecraft/client/gui/components/Button#DEFAULT_NARRATION` | field | 1 |
| `net/minecraft/client/gui/components/Button#active` | field | 1 |
| `net/minecraft/client/gui/components/Button#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/Button#getHeight` | method | 1 |
| `net/minecraft/client/gui/components/Button#getWidth` | method | 1 |
| `net/minecraft/client/gui/components/Button#getX` | method | 1 |
| `net/minecraft/client/gui/components/Button#getY` | method | 1 |
| `net/minecraft/client/gui/components/Button#height` | field | 1 |
| `net/minecraft/client/gui/components/Button#isActive` | method | 2 |
| `net/minecraft/client/gui/components/Button#isHoveredOrFocused` | method | 1 |
| `net/minecraft/client/gui/components/Button#isMouseOver` | method | 2 |
| `net/minecraft/client/gui/components/Button#isValidClickButton` | method | 1 |
| `net/minecraft/client/gui/components/Button#playDownSound` | method | 1 |
| `net/minecraft/client/gui/components/Button#setX` | method | 1 |
| `net/minecraft/client/gui/components/Button#setY` | method | 1 |
| `net/minecraft/client/gui/components/Button#visible` | field | 2 |
| `net/minecraft/client/gui/components/Button#width` | field | 1 |
| `net/minecraft/client/gui/components/ChatComponent#captureClickableText` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent$DisplayMode#FOREGROUND` | field | 1 |
| `net/minecraft/client/gui/components/ChatComponent$DisplayMode#FOREGROUND_RESTRICTED` | field | 1 |
| `net/minecraft/client/gui/components/EditBox#<init>` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#addFormatter` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#canConsumeInput` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#canLoseFocus` | field | 1 |
| `net/minecraft/client/gui/components/EditBox#charTyped` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#extractWidgetRenderState` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#getCursorPosition` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#getValue` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#height` | field | 1 |
| `net/minecraft/client/gui/components/EditBox#isActive` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#isFocused` | method | 5 |
| `net/minecraft/client/gui/components/EditBox#isVisible` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setBordered` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setCanLoseFocus` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setCursorPosition` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setFocused` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setMaxLength` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setResponder` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setTextColor` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setValue` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setX` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setY` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#width` | field | 1 |
| `net/minecraft/client/gui/components/events/ContainerEventHandler#children` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#isFocused` | method | 3 |
| `net/minecraft/client/gui/components/events/GuiEventListener#keyPressed` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#setFocused` | method | 1 |
| `net/minecraft/client/gui/components/toasts/Toast#height` | method | 1 |
| `net/minecraft/client/gui/components/toasts/Toast#width` | method | 1 |
| `net/minecraft/client/gui/components/toasts/ToastManager#visibleToasts` | field | 1 |
| `net/minecraft/client/gui/components/toasts/ToastManager$ToastInstance#getToast` | method | 1 |
| `net/minecraft/client/gui/font/ActiveArea#activeBottom` | method | 1 |
| `net/minecraft/client/gui/font/ActiveArea#activeLeft` | method | 1 |
| `net/minecraft/client/gui/font/ActiveArea#activeRight` | method | 1 |
| `net/minecraft/client/gui/font/ActiveArea#activeTop` | method | 1 |
| `net/minecraft/client/gui/font/ActiveArea#style` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenPosition#<init>` | method | 4 |
| `net/minecraft/client/gui/navigation/ScreenPosition#x` | method | 6 |
| `net/minecraft/client/gui/navigation/ScreenPosition#y` | method | 6 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#<init>` | method | 6 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#bottom` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#containsPoint` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#empty` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#height` | method | 3 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#intersection` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#left` | method | 5 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#position` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#right` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#top` | method | 5 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#transformMaxBounds` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#width` | method | 3 |
| `net/minecraft/client/gui/screens/ChatScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/ChatScreen#getFont` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#extractRenderStateWithTooltipAndSubtitles` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#font` | field | 1 |
| `net/minecraft/client/gui/screens/Screen#getFocused` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#height` | field | 2 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#minecraft` | field | 1 |
| `net/minecraft/client/gui/screens/Screen#onClose` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#removed` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#setFocused` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#tick` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 3 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#SLOT_HIGHLIGHT_BACK_SPRITE` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#SLOT_HIGHLIGHT_FRONT_SPRITE` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractCarriedItem` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#getFont` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#getMenu` | method | 4 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#height` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#hoveredSlot` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#imageHeight` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#imageWidth` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#leftPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#showsActiveEffects` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#topPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#width` | field | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen#recipeBookComponent` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/EffectsInInventory#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/BelowOrAboveWidgetTooltipPositioner#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent#create` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner#positionTooltip` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/DefaultTooltipPositioner#INSTANCE` | field | 2 |
| `net/minecraft/client/gui/screens/options/OptionsSubScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/options/OptionsSubScreen#keyReleased` | method | 1 |
| `net/minecraft/client/gui/screens/options/controls/KeyBindsList#resetMappingAndUpdateButtons` | method | 1 |
| `net/minecraft/client/gui/screens/options/controls/KeyBindsList$KeyEntry#lambda$new$2` | method | 1 |
| `net/minecraft/client/gui/screens/options/controls/KeyBindsScreen#keyPressed` | method | 1 |
| `net/minecraft/client/gui/screens/options/controls/KeyBindsScreen#lambda$addFooter$0` | method | 1 |
| `net/minecraft/client/gui/screens/options/controls/KeyBindsScreen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#getXOrigin` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#getYOrigin` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#isVisible` | method | 2 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#tabButtons` | field | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookTabButton#getHeight` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookTabButton#getWidth` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookTabButton#getX` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookTabButton#getY` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookTabButton#visible` | field | 1 |
| `net/minecraft/client/input/InputQuirks#REPLACE_CTRL_KEY_WITH_CMD_KEY` | field | 2 |
| `net/minecraft/client/input/InputWithModifiers#hasShiftDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#hasShiftDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isEscape` | method | 1 |
| `net/minecraft/client/input/KeyEvent#key` | method | 5 |
| `net/minecraft/client/input/KeyEvent#keycode` | method | 1 |
| `net/minecraft/client/input/KeyEvent#modifiers` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#<init>` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 6 |
| `net/minecraft/client/input/MouseButtonEvent#buttonInfo` | method | 2 |
| `net/minecraft/client/input/MouseButtonEvent#input` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#modifiers` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 7 |
| `net/minecraft/client/multiplayer/ClientLevel#tickRateManager` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#enabledFeatures` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getConnection` | method | 3 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleUpdateRecipes` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#serverBrand` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#handleCreativeModeItemAdd` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData#ip` | field | 1 |
| `net/minecraft/client/multiplayer/ServerData#isLan` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData#isRealm` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData#name` | field | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#handleSystemMessage` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#chatAbilities` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#closeContainer` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#connection` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#containerMenu` | field | 5 |
| `net/minecraft/client/player/LocalPlayer#experienceLevel` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#getActiveEffects` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getInventory` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isCreative` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#sendSystemMessage` | method | 2 |
| `net/minecraft/client/renderer/Rect2i#<init>` | method | 6 |
| `net/minecraft/client/renderer/Rect2i#contains` | method | 3 |
| `net/minecraft/client/renderer/Rect2i#getHeight` | method | 17 |
| `net/minecraft/client/renderer/Rect2i#getWidth` | method | 17 |
| `net/minecraft/client/renderer/Rect2i#getX` | method | 17 |
| `net/minecraft/client/renderer/Rect2i#getY` | method | 17 |
| `net/minecraft/client/renderer/Rect2i#setX` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#setY` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 7 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#getParticleMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#stillMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/FluidStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#bounds` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#ensurePrepared` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#pose` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#scissor` | field | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#byMipLevel` | field | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#getAdditionalMetadata` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#height` | method | 3 |
| `net/minecraft/client/renderer/texture/SpriteContents#width` | method | 3 |
| `net/minecraft/client/renderer/texture/TextureAtlas#getSprite` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#atlasLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#contents` | method | 5 |
| `net/minecraft/client/renderer/texture/TextureManager#register` | method | 1 |
| `net/minecraft/client/resources/language/I18n#get` | method | 1 |
| `net/minecraft/client/resources/language/LanguageManager#getSelected` | method | 1 |
| `net/minecraft/client/resources/metadata/gui/GuiMetadataSection#DEFAULT` | field | 1 |
| `net/minecraft/client/resources/metadata/gui/GuiMetadataSection#TYPE` | field | 1 |
| `net/minecraft/client/resources/metadata/gui/GuiMetadataSection#scaling` | method | 1 |
| `net/minecraft/client/resources/metadata/gui/GuiSpriteScaling$Tile#<init>` | method | 1 |
| `net/minecraft/client/resources/metadata/gui/GuiSpriteScaling$Tile#height` | method | 1 |
| `net/minecraft/client/resources/metadata/gui/GuiSpriteScaling$Tile#width` | method | 1 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/ModelManager#getFluidStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#getAtlasOrThrow` | method | 2 |
| `net/minecraft/client/resources/model/sprite/Material$Baked#sprite` | method | 2 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 1 |
| `net/minecraft/commands/Commands#getDispatcher` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#wrapAsHolder` | method | 1 |
| `net/minecraft/core/Holder#getRegisteredName` | method | 2 |
| `net/minecraft/core/Holder#is` | method | 3 |
| `net/minecraft/core/Holder#isBound` | method | 2 |
| `net/minecraft/core/Holder#kind` | method | 1 |
| `net/minecraft/core/Holder#tags` | method | 2 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 12 |
| `net/minecraft/core/Holder$Kind#REFERENCE` | field | 1 |
| `net/minecraft/core/Holder$Reference#components` | method | 1 |
| `net/minecraft/core/Holder$Reference#is` | method | 3 |
| `net/minecraft/core/Holder$Reference#key` | method | 2 |
| `net/minecraft/core/Holder$Reference#value` | method | 5 |
| `net/minecraft/core/HolderLookup$Provider#lookupOrThrow` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#key` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#listElements` | method | 1 |
| `net/minecraft/core/HolderSet#empty` | method | 1 |
| `net/minecraft/core/HolderSet#iterator` | method | 1 |
| `net/minecraft/core/HolderSet#size` | method | 2 |
| `net/minecraft/core/HolderSet#stream` | method | 1 |
| `net/minecraft/core/HolderSet#unwrapKey` | method | 1 |
| `net/minecraft/core/HolderSet$ListBacked#size` | method | 1 |
| `net/minecraft/core/HolderSet$Named#get` | method | 1 |
| `net/minecraft/core/HolderSet$Named#key` | method | 2 |
| `net/minecraft/core/HolderSet$Named#size` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 3 |
| `net/minecraft/core/NonNullList#size` | method | 3 |
| `net/minecraft/core/Registry#entrySet` | method | 1 |
| `net/minecraft/core/Registry#filterFeatures` | method | 1 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getAny` | method | 1 |
| `net/minecraft/core/Registry#getKey` | method | 6 |
| `net/minecraft/core/Registry#getTagOrEmpty` | method | 1 |
| `net/minecraft/core/Registry#getTags` | method | 3 |
| `net/minecraft/core/Registry#key` | method | 1 |
| `net/minecraft/core/Registry#listElements` | method | 7 |
| `net/minecraft/core/Registry#wrapAsHolder` | method | 1 |
| `net/minecraft/core/Registry$PendingTags#apply` | method | 1 |
| `net/minecraft/core/RegistryAccess#createSerializationContext` | method | 6 |
| `net/minecraft/core/RegistryAccess#freeze` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookup` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 2 |
| `net/minecraft/core/RegistryAccess#registries` | method | 1 |
| `net/minecraft/core/RegistryAccess$Frozen#lookupOrThrow` | method | 1 |
| `net/minecraft/core/RegistryAccess$RegistryEntry#value` | method | 1 |
| `net/minecraft/core/RegistrySetBuilder#<init>` | method | 2 |
| `net/minecraft/core/RegistrySetBuilder#add` | method | 2 |
| `net/minecraft/core/RegistrySetBuilder#build` | method | 2 |
| `net/minecraft/core/component/DataComponentMap#get` | method | 2 |
| `net/minecraft/core/component/DataComponentPatch#CODEC` | field | 2 |
| `net/minecraft/core/component/DataComponentPatch#EMPTY` | field | 3 |
| `net/minecraft/core/component/DataComponentPatch#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch#isEmpty` | method | 3 |
| `net/minecraft/core/component/DataComponentPatch#toString` | method | 2 |
| `net/minecraft/core/component/DataComponentPatch$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#set` | method | 1 |
| `net/minecraft/core/component/DataComponents#ATTRIBUTE_MODIFIERS` | field | 1 |
| `net/minecraft/core/component/DataComponents#BANNER_PATTERNS` | field | 2 |
| `net/minecraft/core/component/DataComponents#BASE_COLOR` | field | 2 |
| `net/minecraft/core/component/DataComponents#BLOCK_STATE` | field | 1 |
| `net/minecraft/core/component/DataComponents#COMPOSTABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#COOKING_FUEL` | field | 1 |
| `net/minecraft/core/component/DataComponents#DYE` | field | 1 |
| `net/minecraft/core/component/DataComponents#EQUIPPABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#FIREWORKS` | field | 3 |
| `net/minecraft/core/component/DataComponents#FIREWORK_EXPLOSION` | field | 6 |
| `net/minecraft/core/component/DataComponents#INSTRUMENT` | field | 1 |
| `net/minecraft/core/component/DataComponents#OMINOUS_BOTTLE_AMPLIFIER` | field | 1 |
| `net/minecraft/core/component/DataComponents#PAINTING_VARIANT` | field | 1 |
| `net/minecraft/core/component/DataComponents#POTION_CONTENTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#POT_DECORATIONS` | field | 1 |
| `net/minecraft/core/component/DataComponents#REPAIRABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#REPAIR_COST` | field | 1 |
| `net/minecraft/core/component/DataComponents#SUSPICIOUS_STEW_EFFECTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#TOOLTIP_STYLE` | field | 1 |
| `net/minecraft/core/component/predicates/PotionsPredicate#potions` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#FLUID` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_SERIALIZER` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_TYPE` | field | 2 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 3 |
| `net/minecraft/core/registries/Registries#CONTEXT_FLOAT_PROVIDER` | field | 1 |
| `net/minecraft/core/registries/Registries#CONTEXT_INT_PROVIDER` | field | 1 |
| `net/minecraft/core/registries/Registries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 2 |
| `net/minecraft/core/registries/Registries#FLUID` | field | 2 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 7 |
| `net/minecraft/core/registries/Registries#POTION` | field | 1 |
| `net/minecraft/core/registries/Registries#PREDICATE` | field | 1 |
| `net/minecraft/core/registries/Registries#RECIPE` | field | 8 |
| `net/minecraft/data/AtlasIds#GUI` | field | 2 |
| `net/minecraft/data/worldgen/BootstrapContext#register` | method | 1 |
| `net/minecraft/gametest/framework/UnknownGameTestException#reason` | field | 1 |
| `net/minecraft/locale/Language#getInstance` | method | 6 |
| `net/minecraft/locale/Language#getVisualOrder` | method | 5 |
| `net/minecraft/locale/Language#has` | method | 1 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 1 |
| `net/minecraft/nbt/Tag#toString` | method | 1 |
| `net/minecraft/nbt/TagParser#parseCompoundFully` | method | 1 |
| `net/minecraft/network/Connection#getLoggableAddress` | method | 1 |
| `net/minecraft/network/Connection#isConnected` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#readEnum` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeEnum` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readEnum` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readUtf` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readVarInt` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeEnum` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeUtf` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeVarInt` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$OpenFile#<init>` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$OpenUrl#<init>` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$RunCommand#<init>` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$RunCommand#command` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#EMPTY` | field | 3 |
| `net/minecraft/network/chat/CommonComponents#SPACE` | field | 1 |
| `net/minecraft/network/chat/Component#EMPTY` | field | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 8 |
| `net/minecraft/network/chat/Component#empty` | method | 9 |
| `net/minecraft/network/chat/Component#getContents` | method | 1 |
| `net/minecraft/network/chat/Component#getSiblings` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 12 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 17 |
| `net/minecraft/network/chat/Component#plainCopy` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 62 |
| `net/minecraft/network/chat/Component#visit` | method | 2 |
| `net/minecraft/network/chat/FormattedText#EMPTY` | field | 2 |
| `net/minecraft/network/chat/FormattedText#composite` | method | 2 |
| `net/minecraft/network/chat/FormattedText#getString` | method | 2 |
| `net/minecraft/network/chat/FormattedText#of` | method | 2 |
| `net/minecraft/network/chat/FormattedText#visit` | method | 2 |
| `net/minecraft/network/chat/HoverEvent$ShowItem#<init>` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowItem#item` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowText#<init>` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getStyle` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 4 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 6 |
| `net/minecraft/network/chat/Style#applyFormat` | method | 1 |
| `net/minecraft/network/chat/Style#equals` | method | 2 |
| `net/minecraft/network/chat/Style#getClickEvent` | method | 3 |
| `net/minecraft/network/chat/Style#getColor` | method | 1 |
| `net/minecraft/network/chat/Style#getHoverEvent` | method | 2 |
| `net/minecraft/network/chat/Style#isBold` | method | 1 |
| `net/minecraft/network/chat/Style#isEmpty` | method | 2 |
| `net/minecraft/network/chat/Style#isItalic` | method | 1 |
| `net/minecraft/network/chat/Style#isObfuscated` | method | 1 |
| `net/minecraft/network/chat/Style#isStrikethrough` | method | 1 |
| `net/minecraft/network/chat/Style#isUnderlined` | method | 1 |
| `net/minecraft/network/chat/Style#withClickEvent` | method | 2 |
| `net/minecraft/network/chat/Style#withColor` | method | 4 |
| `net/minecraft/network/chat/Style#withHoverEvent` | method | 1 |
| `net/minecraft/network/chat/Style#withUnderlined` | method | 1 |
| `net/minecraft/network/chat/TextColor#fromLegacyFormat` | method | 1 |
| `net/minecraft/network/chat/TextColor#fromRgb` | method | 1 |
| `net/minecraft/network/chat/contents/TranslatableContents#getKey` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BOOL` | field | 6 |
| `net/minecraft/network/codec/ByteBufCodecs#FLOAT` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#STRING_UTF8` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#VAR_INT` | field | 8 |
| `net/minecraft/network/codec/StreamCodec#apply` | method | 5 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 14 |
| `net/minecraft/network/codec/StreamCodec#decode` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#encode` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#unit` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 10 |
| `net/minecraft/resources/Identifier#CODEC` | field | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 10 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 31 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 16 |
| `net/minecraft/resources/Identifier#getPath` | method | 14 |
| `net/minecraft/resources/Identifier#hashCode` | method | 2 |
| `net/minecraft/resources/Identifier#toString` | method | 12 |
| `net/minecraft/resources/Identifier#validPathChar` | method | 1 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 3 |
| `net/minecraft/resources/Identifier#withPrefix` | method | 3 |
| `net/minecraft/resources/Identifier#withSuffix` | method | 1 |
| `net/minecraft/resources/RegistryDataLoader#RELOADABLE_REGISTRIES` | field | 1 |
| `net/minecraft/resources/RegistryDataLoader#WORLD_REGISTRIES` | field | 1 |
| `net/minecraft/resources/RegistryDataLoader#load` | method | 1 |
| `net/minecraft/resources/RegistryDataLoader$RegistryData#key` | method | 1 |
| `net/minecraft/resources/RegistryOps#withParent` | method | 1 |
| `net/minecraft/resources/ResourceKey#codec` | method | 1 |
| `net/minecraft/resources/ResourceKey#create` | method | 6 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 18 |
| `net/minecraft/server/MinecraftServer#getCommands` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldPath` | method | 1 |
| `net/minecraft/server/MinecraftServer#storageSource` | field | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 5 |
| `net/minecraft/server/level/ServerPlayer#createCommandSourceStack` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#drop` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getInventory` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getName` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getRandom` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getX` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getY` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getZ` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#inventoryMenu` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#isCreative` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#permissions` | method | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/PackType#SERVER_DATA` | field | 2 |
| `net/minecraft/server/packs/VanillaPackResources#fullResources` | method | 1 |
| `net/minecraft/server/packs/repository/ServerPacksSource#createVanillaPackSource` | method | 1 |
| `net/minecraft/server/packs/resources/CloseableResourceManager#close` | method | 1 |
| `net/minecraft/server/packs/resources/MultiPackResourceManager#<init>` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#openAsReader` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResourceStack` | method | 1 |
| `net/minecraft/server/permissions/PermissionSet#hasPermission` | method | 1 |
| `net/minecraft/server/permissions/Permissions#COMMANDS_GAMEMASTER` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_PICKUP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#UI_BUTTON_CLICK` | field | 2 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#CURSE` | field | 2 |
| `net/minecraft/tags/TagKey#create` | method | 4 |
| `net/minecraft/tags/TagKey#getName` | method | 1 |
| `net/minecraft/tags/TagKey#location` | method | 5 |
| `net/minecraft/tags/TagKey#registry` | method | 2 |
| `net/minecraft/tags/TagLoader#buildUpdatedLookups` | method | 1 |
| `net/minecraft/tags/TagLoader#loadTagsForExistingRegistries` | method | 1 |
| `net/minecraft/util/ARGB#alpha` | method | 1 |
| `net/minecraft/util/ARGB#blue` | method | 1 |
| `net/minecraft/util/ARGB#green` | method | 1 |
| `net/minecraft/util/ARGB#red` | method | 1 |
| `net/minecraft/util/FileUtil#isPathPartPortable` | method | 1 |
| `net/minecraft/util/FileUtil#sanitizeName` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#EMPTY` | field | 2 |
| `net/minecraft/util/FormattedCharSequence#composite` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#forward` | method | 1 |
| `net/minecraft/util/Prediction#SERVER_ONLY` | field | 2 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 1 |
| `net/minecraft/util/Util#getMillis` | method | 1 |
| `net/minecraft/util/context/ContextMap#builder` | method | 1 |
| `net/minecraft/util/context/ContextMap$Builder#build` | method | 1 |
| `net/minecraft/util/context/ContextMap$Builder#set` | method | 1 |
| `net/minecraft/world/TickRateManager#tickrate` | method | 1 |
| `net/minecraft/world/effect/MobEffect#getDisplayName` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getAmplifier` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 1 |
| `net/minecraft/world/entity/EntityEquipment#<init>` | method | 2 |
| `net/minecraft/world/entity/EquipmentSlot#CHEST` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#getIndex` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR` | field | 1 |
| `net/minecraft/world/entity/item/ItemEntity#makeFakeItem` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setNoPickUpDelay` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setTarget` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#<init>` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getNonEquipmentItems` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#isHotbarSlot` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#setItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#containerMenu` | field | 4 |
| `net/minecraft/world/entity/player/Player#drop` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendSystemMessage` | method | 1 |
| `net/minecraft/world/flag/FeatureFlagSet#of` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#broadcastChanges` | method | 2 |
| `net/minecraft/world/inventory/AbstractContainerMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getCarried` | method | 7 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getSlot` | method | 3 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getType` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#setCarried` | method | 3 |
| `net/minecraft/world/inventory/AbstractContainerMenu#slots` | field | 3 |
| `net/minecraft/world/inventory/AbstractFurnaceMenu#getSlot` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#calculateIncreasedRepairCost` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#getCost` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#getResultSlot` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#slots` | field | 2 |
| `net/minecraft/world/inventory/GrindstoneMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/GrindstoneMenu#computeResult` | method | 1 |
| `net/minecraft/world/inventory/InventoryMenu#broadcastChanges` | method | 1 |
| `net/minecraft/world/inventory/MenuType#ANVIL` | field | 1 |
| `net/minecraft/world/inventory/MenuType#BLAST_FURNACE` | field | 1 |
| `net/minecraft/world/inventory/MenuType#BREWING_STAND` | field | 1 |
| `net/minecraft/world/inventory/MenuType#CRAFTER_3x3` | field | 1 |
| `net/minecraft/world/inventory/MenuType#CRAFTING` | field | 1 |
| `net/minecraft/world/inventory/MenuType#FURNACE` | field | 1 |
| `net/minecraft/world/inventory/MenuType#SMITHING` | field | 1 |
| `net/minecraft/world/inventory/MenuType#SMOKER` | field | 1 |
| `net/minecraft/world/inventory/Slot#allowModification` | method | 2 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 5 |
| `net/minecraft/world/inventory/Slot#getMaxStackSize` | method | 1 |
| `net/minecraft/world/inventory/Slot#index` | field | 8 |
| `net/minecraft/world/inventory/Slot#isFake` | method | 2 |
| `net/minecraft/world/inventory/Slot#mayPickup` | method | 1 |
| `net/minecraft/world/inventory/Slot#mayPlace` | method | 1 |
| `net/minecraft/world/inventory/Slot#safeTake` | method | 1 |
| `net/minecraft/world/inventory/Slot#set` | method | 2 |
| `net/minecraft/world/inventory/Slot#x` | field | 1 |
| `net/minecraft/world/inventory/Slot#y` | field | 1 |
| `net/minecraft/world/item/BannerItem#getColor` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 3 |
| `net/minecraft/world/item/CreativeModeTab#buildContents` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab#contains` | method | 2 |
| `net/minecraft/world/item/CreativeModeTab#getDisplayItems` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab#getDisplayName` | method | 3 |
| `net/minecraft/world/item/CreativeModeTab#getSearchTabDisplayItems` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab#getType` | method | 3 |
| `net/minecraft/world/item/CreativeModeTab#shouldDisplay` | method | 2 |
| `net/minecraft/world/item/CreativeModeTab$ItemDisplayParameters#<init>` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Type#CATEGORY` | field | 3 |
| `net/minecraft/world/item/CreativeModeTabs#allTabs` | method | 2 |
| `net/minecraft/world/item/DyeColor#BLUE` | field | 1 |
| `net/minecraft/world/item/DyeColor#RED` | field | 1 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 2 |
| `net/minecraft/world/item/DyeColor#YELLOW` | field | 1 |
| `net/minecraft/world/item/DyeColor#getFireworkColor` | method | 2 |
| `net/minecraft/world/item/FireworkRocketItem#CRAFTABLE_DURATIONS` | field | 2 |
| `net/minecraft/world/item/Item#CODEC` | field | 1 |
| `net/minecraft/world/item/Item#builtInRegistryHolder` | method | 2 |
| `net/minecraft/world/item/Item#components` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 7 |
| `net/minecraft/world/item/Item#getDescriptionId` | method | 3 |
| `net/minecraft/world/item/Item$TooltipContext#EMPTY` | field | 2 |
| `net/minecraft/world/item/Item$TooltipContext#of` | method | 3 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 19 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 14 |
| `net/minecraft/world/item/ItemStack#STREAM_CODEC` | field | 3 |
| `net/minecraft/world/item/ItemStack#canBeEnchantedWith` | method | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 12 |
| `net/minecraft/world/item/ItemStack#copyWithCount` | method | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 7 |
| `net/minecraft/world/item/ItemStack#getComponentsPatch` | method | 4 |
| `net/minecraft/world/item/ItemStack#getCreatorNamespace` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDisplayName` | method | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 15 |
| `net/minecraft/world/item/ItemStack#getMaxDamage` | method | 3 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 3 |
| `net/minecraft/world/item/ItemStack#getOrDefault` | method | 5 |
| `net/minecraft/world/item/ItemStack#getTooltipImage` | method | 1 |
| `net/minecraft/world/item/ItemStack#getTooltipLines` | method | 2 |
| `net/minecraft/world/item/ItemStack#grow` | method | 1 |
| `net/minecraft/world/item/ItemStack#has` | method | 2 |
| `net/minecraft/world/item/ItemStack#is` | method | 1 |
| `net/minecraft/world/item/ItemStack#isDamageableItem` | method | 2 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 26 |
| `net/minecraft/world/item/ItemStack#isEnchantable` | method | 2 |
| `net/minecraft/world/item/ItemStack#isSameItem` | method | 3 |
| `net/minecraft/world/item/ItemStack#isStackable` | method | 1 |
| `net/minecraft/world/item/ItemStack#set` | method | 6 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 5 |
| `net/minecraft/world/item/ItemStack#setDamageValue` | method | 2 |
| `net/minecraft/world/item/ItemStack#typeHolder` | method | 3 |
| `net/minecraft/world/item/ItemStack#validateStrict` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStackTemplate#STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStackTemplate#apply` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#components` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#count` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#fromNonEmptyStack` | method | 7 |
| `net/minecraft/world/item/ItemStackTemplate#typeHolder` | method | 1 |
| `net/minecraft/world/item/Items#APPLE` | field | 1 |
| `net/minecraft/world/item/Items#ARROW` | field | 1 |
| `net/minecraft/world/item/Items#BLAST_FURNACE` | field | 1 |
| `net/minecraft/world/item/Items#BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#CRAFTING_TABLE` | field | 1 |
| `net/minecraft/world/item/Items#DECORATED_POT` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#ENCHANTED_BOOK` | field | 2 |
| `net/minecraft/world/item/Items#FIREWORK_ROCKET` | field | 3 |
| `net/minecraft/world/item/Items#FIREWORK_STAR` | field | 3 |
| `net/minecraft/world/item/Items#FURNACE` | field | 2 |
| `net/minecraft/world/item/Items#GOAT_HORN` | field | 1 |
| `net/minecraft/world/item/Items#GRASS_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#LIGHT` | field | 1 |
| `net/minecraft/world/item/Items#LINGERING_POTION` | field | 2 |
| `net/minecraft/world/item/Items#NAME_TAG` | field | 1 |
| `net/minecraft/world/item/Items#OMINOUS_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#PAINTING` | field | 1 |
| `net/minecraft/world/item/Items#POTION` | field | 2 |
| `net/minecraft/world/item/Items#SHIELD` | field | 1 |
| `net/minecraft/world/item/Items#SMOKER` | field | 1 |
| `net/minecraft/world/item/Items#SPLASH_POTION` | field | 1 |
| `net/minecraft/world/item/Items#SUSPICIOUS_STEW` | field | 1 |
| `net/minecraft/world/item/Items#TIPPED_ARROW` | field | 2 |
| `net/minecraft/world/item/Items#WATER_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#WET_SPONGE` | field | 1 |
| `net/minecraft/world/item/TooltipFlag#isAdvanced` | method | 1 |
| `net/minecraft/world/item/TooltipFlag#isCreative` | method | 1 |
| `net/minecraft/world/item/TooltipFlag$Default#ADVANCED` | field | 2 |
| `net/minecraft/world/item/TooltipFlag$Default#NORMAL` | field | 3 |
| `net/minecraft/world/item/TooltipFlag$Default#asCreative` | method | 2 |
| `net/minecraft/world/item/alchemy/PotionContents#EMPTY` | field | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#createItemStack` | method | 3 |
| `net/minecraft/world/item/alchemy/PotionContents#potion` | method | 1 |
| `net/minecraft/world/item/alchemy/Potions#WATER` | field | 1 |
| `net/minecraft/world/item/component/BlockItemStateProperties#get` | method | 1 |
| `net/minecraft/world/item/component/Compostable#layers` | method | 1 |
| `net/minecraft/world/item/component/CookingFuel#burnTime` | method | 1 |
| `net/minecraft/world/item/component/CookingFuel#speedMultiplier` | method | 1 |
| `net/minecraft/world/item/component/FireworkExplosion#<init>` | method | 2 |
| `net/minecraft/world/item/component/FireworkExplosion#CODEC` | field | 1 |
| `net/minecraft/world/item/component/FireworkExplosion#DEFAULT` | field | 2 |
| `net/minecraft/world/item/component/FireworkExplosion#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/component/FireworkExplosion#colors` | method | 3 |
| `net/minecraft/world/item/component/FireworkExplosion#equals` | method | 1 |
| `net/minecraft/world/item/component/FireworkExplosion#fadeColors` | method | 2 |
| `net/minecraft/world/item/component/FireworkExplosion#hasTrail` | method | 2 |
| `net/minecraft/world/item/component/FireworkExplosion#hasTwinkle` | method | 2 |
| `net/minecraft/world/item/component/FireworkExplosion#shape` | method | 2 |
| `net/minecraft/world/item/component/FireworkExplosion#withFadeColors` | method | 1 |
| `net/minecraft/world/item/component/FireworkExplosion$Shape#SMALL_BALL` | field | 3 |
| `net/minecraft/world/item/component/FireworkExplosion$Shape#values` | method | 1 |
| `net/minecraft/world/item/component/Fireworks#<init>` | method | 2 |
| `net/minecraft/world/item/component/Fireworks#explosions` | method | 1 |
| `net/minecraft/world/item/component/Fireworks#flightDuration` | method | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#EMPTY` | field | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#compute` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#cookingTime` | method | 2 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#display` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#experience` | method | 2 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe$CookingBookInfo#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/BrewingRecipe#getInput` | method | 1 |
| `net/minecraft/world/item/crafting/BrewingRecipe#getOutput` | method | 1 |
| `net/minecraft/world/item/crafting/BrewingRecipe#getReagent` | method | 1 |
| `net/minecraft/world/item/crafting/CookingBookCategory#MISC` | field | 1 |
| `net/minecraft/world/item/crafting/CraftingBookCategory#CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/CraftingBookCategory#MISC` | field | 3 |
| `net/minecraft/world/item/crafting/CraftingInput#of` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#display` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingRecipe#isSpecial` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/FireworkRocketRecipe#fuel` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkRocketRecipe#result` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkRocketRecipe#shell` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkRocketRecipe#star` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarFadeRecipe#dye` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarFadeRecipe#result` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarFadeRecipe#target` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarRecipe#dye` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarRecipe#fuel` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarRecipe#result` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarRecipe#shapes` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarRecipe#trail` | field | 1 |
| `net/minecraft/world/item/crafting/FireworkStarRecipe#twinkle` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CONTENTS_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#acceptsItem` | method | 2 |
| `net/minecraft/world/item/crafting/Ingredient#display` | method | 8 |
| `net/minecraft/world/item/crafting/Ingredient#isEmpty` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#items` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#of` | method | 4 |
| `net/minecraft/world/item/crafting/Ingredient#test` | method | 2 |
| `net/minecraft/world/item/crafting/PlacementInfo#createFromOptionals` | method | 1 |
| `net/minecraft/world/item/crafting/PotionIngredient#ingredient` | method | 1 |
| `net/minecraft/world/item/crafting/PotionIngredient#potions` | method | 1 |
| `net/minecraft/world/item/crafting/PotionIngredient#test` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe#DIRECT_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Recipe$CommonInfo#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeHolder#<init>` | method | 5 |
| `net/minecraft/world/item/crafting/RecipeHolder#equals` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#id` | method | 12 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 18 |
| `net/minecraft/world/item/crafting/RecipeMap#EMPTY` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeMap#byKey` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeMap#byType` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeMap#create` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeMap#values` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeSerializer#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeType#BLASTING` | field | 4 |
| `net/minecraft/world/item/crafting/RecipeType#BREWING` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeType#CAMPFIRE_COOKING` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeType#CRAFTING` | field | 3 |
| `net/minecraft/world/item/crafting/RecipeType#SMELTING` | field | 3 |
| `net/minecraft/world/item/crafting/RecipeType#SMITHING` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeType#SMOKING` | field | 4 |
| `net/minecraft/world/item/crafting/RecipeType#STONECUTTING` | field | 2 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#MAP_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#height` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#matches` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#of` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#width` | method | 1 |
| `net/minecraft/world/item/crafting/ShapelessRecipe#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/ShieldDecorationRecipe#banner` | field | 1 |
| `net/minecraft/world/item/crafting/ShieldDecorationRecipe#result` | field | 1 |
| `net/minecraft/world/item/crafting/ShieldDecorationRecipe#target` | field | 1 |
| `net/minecraft/world/item/crafting/SingleItemRecipe#input` | method | 1 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/SmeltingRecipe#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/SmeltingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/SmeltingRecipe#cookingTime` | method | 2 |
| `net/minecraft/world/item/crafting/SmeltingRecipe#experience` | method | 2 |
| `net/minecraft/world/item/crafting/SmeltingRecipe#input` | method | 2 |
| `net/minecraft/world/item/crafting/SmeltingRecipe#result` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingRecipeInput#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/SmithingTransformRecipe#addition` | field | 1 |
| `net/minecraft/world/item/crafting/SmithingTransformRecipe#base` | field | 1 |
| `net/minecraft/world/item/crafting/SmithingTransformRecipe#template` | field | 1 |
| `net/minecraft/world/item/crafting/SmithingTrimRecipe#addition` | field | 1 |
| `net/minecraft/world/item/crafting/SmithingTrimRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingTrimRecipe#base` | field | 1 |
| `net/minecraft/world/item/crafting/SmithingTrimRecipe#template` | field | 1 |
| `net/minecraft/world/item/crafting/StonecutterRecipe#display` | method | 1 |
| `net/minecraft/world/item/crafting/StonecutterRecipe#isSpecial` | method | 1 |
| `net/minecraft/world/item/crafting/TransmuteRecipe#createWithOriginalComponents` | method | 1 |
| `net/minecraft/world/item/crafting/display/DisplayContentsFactory$ForStacks#forStack` | method | 1 |
| `net/minecraft/world/item/crafting/display/FurnaceRecipeDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/FurnaceRecipeDisplay#duration` | method | 1 |
| `net/minecraft/world/item/crafting/display/FurnaceRecipeDisplay#experience` | method | 1 |
| `net/minecraft/world/item/crafting/display/FurnaceRecipeDisplay#fuel` | method | 1 |
| `net/minecraft/world/item/crafting/display/FurnaceRecipeDisplay#ingredient` | method | 1 |
| `net/minecraft/world/item/crafting/display/FurnaceRecipeDisplay#result` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplay#result` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#height` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#width` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapelessCraftingRecipeDisplay#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#CODEC` | field | 4 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#STREAM_CODEC` | field | 4 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#isEnabled` | method | 2 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolve` | method | 4 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolveForFirstStack` | method | 2 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolveForStacks` | method | 3 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#type` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$AnyFuel#INSTANCE` | field | 3 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$AnyFuel#TYPE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Composite#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Composite#TYPE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Composite#contents` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Empty#INSTANCE` | field | 4 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemSlotDisplay#<init>` | method | 3 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemSlotDisplay#TYPE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemSlotDisplay#item` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemStackContentsFactory#INSTANCE` | field | 2 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemStackSlotDisplay#<init>` | method | 5 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$TagSlotDisplay#TYPE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$TagSlotDisplay#tag` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Type#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Type#equals` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$WithAnyPotion#TYPE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$WithRemainder#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$WithRemainder#TYPE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$WithRemainder#input` | method | 3 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$WithRemainder#remainder` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplayContext#fromLevel` | method | 1 |
| `net/minecraft/world/item/crafting/display/StonecutterRecipeDisplay#input` | method | 1 |
| `net/minecraft/world/item/crafting/display/StonecutterRecipeDisplay#result` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#getMaxLevel` | method | 2 |
| `net/minecraft/world/item/enchantment/Enchantment#getMinCost` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#getSupportedItems` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#canStoreEnchantments` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#getEnchantmentsForCrafting` | method | 3 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#setEnchantments` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#entrySet` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#getLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#isEmpty` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#keySet` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments$Mutable#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments$Mutable#set` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments$Mutable#toImmutable` | method | 1 |
| `net/minecraft/world/item/enchantment/Repairable#items` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable#slot` | method | 1 |
| `net/minecraft/world/level/ItemLike#asItem` | method | 6 |
| `net/minecraft/world/level/block/Block#builtInRegistryHolder` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 3 |
| `net/minecraft/world/level/block/Block#getName` | method | 2 |
| `net/minecraft/world/level/block/Blocks#ANVIL` | field | 2 |
| `net/minecraft/world/level/block/Blocks#BLAST_FURNACE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#BREWING_STAND` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CAMPFIRE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#COMPOSTER` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CRAFTER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRAFTING_TABLE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#FURNACE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#GRINDSTONE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SMITHING_TABLE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SMOKER` | field | 3 |
| `net/minecraft/world/level/block/Blocks#SOUL_CAMPFIRE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONECUTTER` | field | 2 |
| `net/minecraft/world/level/block/LightBlock#LEVEL` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#typeHolder` | method | 1 |
| `net/minecraft/world/level/material/Fluid#builtInRegistryHolder` | method | 2 |
| `net/minecraft/world/level/material/Fluid#defaultFluidState` | method | 3 |
| `net/minecraft/world/level/material/Fluid#getBucket` | method | 1 |
| `net/minecraft/world/level/material/Fluid#isSame` | method | 1 |
| `net/minecraft/world/level/material/Fluid#isSource` | method | 3 |
| `net/minecraft/world/level/material/Fluids#EMPTY` | field | 1 |
| `net/minecraft/world/level/material/Fluids#WATER` | field | 1 |
| `net/minecraft/world/level/storage/LevelResource#ROOT` | field | 1 |
| `net/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess#getLevelId` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootContext#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_STATE` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootPredicates#bootstrap` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ContextFloatProviders#bootstrap` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ResolvableFloat#getFromItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#bootstrap` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt#getFromItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt$Constant#value` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt$Reference#key` | method | 1 |
| `net/minecraft/world/phys/Vec2#<init>` | method | 3 |
| `net/minecraft/world/phys/Vec2#x` | field | 4 |
| `net/minecraft/world/phys/Vec2#y` | field | 4 |

</details>

