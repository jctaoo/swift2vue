import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "SecureField",
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
      <label class="text-field secure-filed built-in" :ref="(el) => setRef(el)">
        <slot></slot>
        <input type="password" v-model="text" />
      </label>
    </template>
  </SwiftCommon>`
};
