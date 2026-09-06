<script setup lang="ts">
import { ref } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { Gui } from '../stores/styles';

const props = defineProps<{ gui: Gui; selected: boolean }>();
const emit = defineEmits<{ select: [gui: Gui] }>();

const HOLD_MS = 2000;
const progress = ref(0);
const holding = ref(false);

let startedAt = 0;
let rafId: number | null = null;
let timeoutId: ReturnType<typeof setTimeout> | null = null;

function startHold() {
    holding.value = true;
    startedAt = performance.now();
    tick();
    timeoutId = setTimeout(() => {
        emit('select', props.gui);
        cancelHold();
    }, HOLD_MS);
}

function tick() {
    if (!holding.value) return;
    progress.value = Math.min(100, ((performance.now() - startedAt) / HOLD_MS) * 100);
    if (progress.value < 100) rafId = requestAnimationFrame(tick);
}

function cancelHold() {
    holding.value = false;
    progress.value = 0;
    if (timeoutId) { clearTimeout(timeoutId); timeoutId = null; }
    if (rafId) { cancelAnimationFrame(rafId); rafId = null; }
}
</script>

<template>
<li
    class="listless ata-option-big palette-dark-empty gui-option"
    :class="{ selected: props.selected }"
    @pointerdown="startHold"
    @pointerup="cancelHold"
    @pointerleave="cancelHold"
    @pointercancel="cancelHold"
>
    <div class="hold-progress" :style="{ width: progress + '%' }"></div>
    <img v-if="props.gui.icon" class="gui-icon" :src="convertFileSrc(props.gui.icon)" alt="" />
    <span class="ata-h3">{{ props.gui.name }}</span>
</li>
</template>

<style scoped lang="scss">
.gui-option {
    position: relative;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
}
.hold-progress {
    position: absolute;
    inset: 0;
    background-color: $ata-accent-tertiary;
    opacity: 0.5;
    pointer-events: none;
}
.gui-icon {
    width: 32px;
    height: 32px;
    object-fit: contain;
}
</style>