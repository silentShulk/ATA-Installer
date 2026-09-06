<script lang="ts" setup>
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Select from "./Components/Select.vue";
import InstallationState from "./Components/InstallationState.vue";
import TitleBar from "./Components/TitleBar.vue";
import { useStateStore } from "./stores/state"
import { useStylesStore } from "./stores/styles"
import type { Gui } from "./stores/styles";
import "./style/components/button.scss"
import { open } from '@tauri-apps/plugin-dialog';
import { commands } from "./bindings";



const stateStore = useStateStore()
const stylesStore = useStylesStore()

async function refreshInstallationState() {
    stateStore.installationState = await invoke("check_installation_state");
}

async function refreshGuis() {
    const [selected, available] = await invoke<[Gui | null, Gui[]]>("get_guis");
    stylesStore.selectedGui = selected;
    stylesStore.availableGuis = available;
}

async function checks() {
    await refreshInstallationState();
    await refreshGuis();
}

async function createFolders() {
    await invoke("create_folders")
    await refreshInstallationState()
}
async function createExecutable() {
    await invoke("extract_tools")
    await refreshInstallationState()
    await refreshGuis()
}
async function createDefaultData() {
    await invoke("create_default_data")
    await refreshInstallationState()
}
async function createDefaultSettings() {
    await invoke("create_default_settings")
    await refreshInstallationState()
}

async function launchGui(gui: Gui) {
    await invoke('set_selected_style', { selectedStyle: gui.name })
    await invoke('launch_gui', { gui })
    await refreshGuis()
}

async function addGui() {
    const paths = await commands.getPaths();

    const pathToNewGui = await open({
        multiple: false,
        directory: false,
        defaultPath: paths.downloads,
        filters: [{
            name: "",
            extensions: ['exe']
        }]
    });
    await invoke('add_gui', { pathToNewGui: pathToNewGui })

    await refreshGuis()
}
async function removeGui() {
    const paths = await commands.getPaths();

    const pathInsideGuiFolder = await open({
        multiple: false,
        directory: false,
        defaultPath: paths.uis_dir,
        filters: [{
            name: "",
            extensions: ['exe']
        }]
    });
    await invoke('remove_gui', { pathInsideGuiFolder: pathInsideGuiFolder })

    await refreshGuis()
}

onMounted(async () => {
    checks()
});
</script>



<template>
    <TitleBar />
    <div id="ata-app">
        <header class="palette-main">
            <h1 class="spaceless ata-h1"> ATA Launcher </h1>
        </header>

        <InstallationState :state="stateStore.installationState"
            @create-folders="createFolders"
            @create-executable="createExecutable"
            @create-default-data="createDefaultData"
            @create-default-settings="createDefaultSettings" />

        <main id="style" class="ata-main justify-space-evenly">
            <button class="ata-btn-medium-big palette-dark-bad ata-h2 centered-self-v" @click="removeGui"> Remove Style</button>
            <div id="style-selector">
                <Select :guis="stylesStore.availableGuis" :selectedGui="stylesStore.selectedGui" @launch="launchGui"/>
            </div>
            <button class="ata-btn-medium-big palette-dark-good ata-h2 centered-self-v" @click="addGui"> Add Style </button>
        </main>
    </div>
</template>



<style lang="scss">
#ata-app {
    display: flex;
    flex-direction: column;

    height: calc(100vh - 32px);
    width: 100vw;

    background-color: $ata-main;
    font-family: Jetbrains Mono;

    overflow: hidden;
    flex: 1;
    min-height: 0;

    border: 5px solid $ata-accent;
    box-sizing: border-box;

    border-radius: 0 0 15px 15px;
}

#style {
    height: 60%;
    width: 100%;
}

.ata-main {
    display: flex;
    flex-grow: 1;

    min-height: 0;
    margin: 10px;

    gap: 10px;
}

.ata-grid {
    display: grid;
    flex-grow: 1;
    grid-auto-flow: column;

    place-items: center;

    gap: 10px;
}


.truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    word-wrap: normal;
}
.spaceless {
    margin: 0;
    padding: 0;
}

.flex {
    display: flex;
    align-items: center;
    gap: 10px;
}
.flex-column {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.justify-center {
    justify-content: center;
}
.justify-space-evenly {
    justify-content: space-evenly;
}
.centered-self-v {
    display: flex;
    align-self: center;
}

body,
html {
    margin: 0;
    padding: 0;
}
</style>