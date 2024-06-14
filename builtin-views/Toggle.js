import SwiftCommon from './SwiftCommon.js'
import Text from './Text.js'

export default {
  components: { SwiftCommon, Text },
  name: 'Toggle',
  props: ['isOn', 'title'],
  emits: ['update:isOn'],
  setup(props, { emit }) {
    const toggleOn = computed({
      get: () => props.isOn,
      set: (val) => emit('update:isOn', val),
    })

    const showTitle = computed(() => {
      return props.title !== undefined
    })

    return {
      showTitle,
      active: toggleOn,
    }
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <label class="toggle built-in" :ref="(el) => setRef(el)">
        <span v-if="showTitle">
          <Text>{{ title }}</Text>
        </span>
        <n-switch v-model:value="active" />
      </label>
    </template>
  </SwiftCommon>`,
}
