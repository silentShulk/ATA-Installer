<script setup lang="ts">
import { ref, computed } from 'vue';
import { Fzf } from 'fzf';
import GuiOption from './GuiOption.vue';
import type { Gui } from '../stores/styles';
import "../style/components/option.scss"
import "../style/components/select.scss"
import "../style/components/text-input.scss"

const props = defineProps<{
    guis: Gui[];
    selectedGui: Gui | null;
}>();

const query = ref('');
const fzf = computed(() => new Fzf(props.guis, {
    selector: (g: Gui) => g.name,
    fuzzy: "v2"
}));

const filteredGuis = computed(() => {
    if (!query.value) return props.guis;
    return fzf.value.find(query.value).map(entry => entry.item);
});
const filter = (e: Event) => {
    query.value = (e.target as HTMLInputElement).value;
};

defineEmits<{ launch: [gui: Gui] }>();
</script>

<template>
<main id="selector" class="ata-select-big palette-gradient-main-accent">
    <input
    class="ata-input-big-top palette-accent ata-h1"
    placeholder="Search style"
    @input="filter"
    />
    <ul id="style-list" class="justify-center">
        <GuiOption
        v-if="props.selectedGui"
        :gui="props.selectedGui"
        :selected="true"
        @select="g => $emit('launch', g)"
        />
        <GuiOption
        v-for="g in filteredGuis"
        :key="g.name"
        :gui="g"
        :selected="false"
        @select="g => $emit('launch', g)"
        />
    </ul>
</main>
</template>

<style scoped lang="scss">
#selector {
    max-height: 90%;
    display: flex;
    flex-direction: column;
}
#style-list {
    padding: 0;
    margin: 0;
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
}
.listless {
    list-style: none;
    margin: 0;
    padding: 0;
}
</style>