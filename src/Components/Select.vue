<script setup lang="ts">
import { ref, computed } from 'vue';
import { Fzf } from 'fzf';
import "../style/components/option.scss"
import "../style/components/select.scss"
import "../style/components/text-input.scss"

const props = defineProps<{
    elements: string[];
    selectedElement: string;
}>();

const query = ref('');
const fzf = computed(() => new Fzf(props.elements, {
    selector: (e: string) => e,
    fuzzy: "v2"
}));

const filteredElements = computed(() => {
    if (!query.value) return props.elements;
    return fzf.value.find(query.value).map(entry => entry.item);
});
const filter = (e: Event) => {
    query.value = (e.target as HTMLInputElement).value;
};

const emit = defineEmits<{
    newSelection: [selection: string]
}>();
</script>



<template>
<main id="selector" class="ata-select-big palette-gradient-main-accent">
    <input
    class="ata-input-big-top palette-accent ata-h1"
    placeholder="Search style"
    @input="filter"
    />
    <ul id="style-list" class="justify-center">
        <li class="listless ata-option-big palette-dark-empty">
            <input
            type="radio"
            class="palette-accent"
            :checked=true
            />
            <span class="ata-h3">{{ props.selectedElement }}</span>
        </li>
        <li
        v-for="e in filteredElements"
        :key="e"
        class="listless ata-option-big palette-dark-empty"
        >
            <input
            type="radio"
            :checked=false
            class="palette-accent"
            @change="$emit('newSelection', e)"
            />
            <span class="ata-h3">{{ e }}</span>
        </li>
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
    padding:0;
    margin:0;

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
