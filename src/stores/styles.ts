import { ref } from "vue";
import { defineStore } from "pinia";

export type GuiType = "Webapp" | "App";

export interface Gui {
    name: string;
    kind: GuiType;
    icon: string | null;
    executable: string;
}

export const useStylesStore = defineStore('style', () => {
    const availableGuis = ref<Gui[]>([]);
    const selectedGui = ref<Gui | null>(null);

    return { availableGuis, selectedGui }
})