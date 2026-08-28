<script lang="ts" setup>
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Select from "./Components/Select.vue";
import InstallationState from "./Components/InstallationState.vue";
import { useStateStore } from "./stores/state"
import { useStylesStore } from "./stores/styles"
import "./style/components/button.scss"
import { open } from '@tauri-apps/plugin-dialog';
import { commands } from "./bindings";



const stateStore = useStateStore()
const stylesStore = useStylesStore()

async function refreshInstallationState() {
    stateStore.installationState = await invoke("check_installation_state");
}

async function refreshStyles() {
    stylesStore.avaiableStyles = await invoke("scan_for_styles")
    stylesStore.selectedStyle = await invoke("get_selected_style");
}

async function checks() {
    await refreshInstallationState();
    await refreshStyles();
}

async function createFolders() {
    await invoke("create_folders")
    await refreshInstallationState();
}
async function createDefaultData() {
    await invoke("create_default_data")
    await refreshInstallationState();
}
async function createDefaultSettings() {
    await invoke("create_default_settings")
    await refreshInstallationState();
}

async function changeSelectedStyle(selectedStyle: string) {
    await invoke('set_selected_style', { selectedStyle: selectedStyle})
    await refreshStyles()
}

async function addStyle() {
    const paths = await commands.getPaths();

    const pathToNewStyle = await open({
        multiple: false,
        directory: false,
        defaultPath: paths.downloads,
        filters: [{
            name: "",
            extensions: ['exe']
        }]
    });
    await invoke('add_style', { pathToNewStyle: pathToNewStyle })

    await refreshStyles()
}
async function removeStyle() {
    const paths = await commands.getPaths();
  
    const pathToStyleToRemove = await open({
        multiple: false,
        directory: false,
        defaultPath: paths.uis_dir,
        filters: [{
            name: "",
            extensions: ['exe']
        }]
    });
    await invoke('remove_style', { pathToStyleToRemove: pathToStyleToRemove })
  
    await refreshStyles()
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
            @create-default-data="createDefaultData"
            @create-default-settings="createDefaultSettings" />

        <main id="style" class="ata-main justify-space-evenly">
            <button class="ata-btn-medium-big palette-dark-bad ata-h2 centered-self-v" @click="removeStyle"> Remove Style</button>
            <div id="style-selector">
                <Select :elements="stylesStore.avaiableStyles" :selectedElement="stylesStore.selectedStyle" @newSelection="changeSelectedStyle"/>
            </div>
            <button class="ata-btn-medium-big palette-dark-good ata-h2 centered-self-v" @click="addStyle"> Add Style </button>
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
