import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "TextField",
  props: ['text'],
  emits: ['update:text'],
  setup(props, { emit }) {
    const text = computed({
      get: () => props.text,
      set: (val) => emit('update:text', val),
    })
    return { text }
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <label class="text-field built-in" :ref="(el) => setRef(el)">
        <slot></slot>
        <input type="text" v-model="text" />
      </label>
    </template>
  </SwiftCommon>`
};
