<script setup lang="ts">
import "../style/components/button.scss"

defineProps<{
    state: [boolean, boolean, boolean, boolean]
}>()

const emit = defineEmits<{
    createFolders: []
    createExecutable: []
    createDefaultData: []
    createDefaultSettings: []
}>()
</script>



<template>
    <div id="installation-components" class="ata-grid">
        <button id="folders" class="ata-btn-medium-small"
            :class="{ 'palette-enabled': state[0], 'palette-disabled': !state[0] }" @click="emit('createFolders')">
            Folders </button>
        <button id="executable" class="ata-btn-medium-small"
            :class="{ 'palette-enabled': state[1], 'palette-disabled': !state[1] }" @click="emit('createExecutable')"> Executable
        </button>
        <button id="data" class="ata-btn-medium-small"
            :class="{ 'palette-enabled': state[2], 'palette-disabled': !state[2] }" @click="emit('createDefaultData')">
            Data File </button>
        <button id="settings" class="ata-btn-medium-small"
            :class="{ 'palette-enabled': state[3], 'palette-disabled': !state[3] }"
            @click="emit('createDefaultSettings')"> Settings File </button>
    </div>
</template>



<style lang="scss">
#executable {
    grid-area: executable;
}
#folders {
    grid-area: folders;
}
#data {
    grid-area: data;
}
#settings {
    grid-area: settings;
}

#installation-components {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: 1;
    grid-template-areas:
        "folders executable data settings";
}
</style>
