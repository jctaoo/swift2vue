{
  /* <VStack>
  <ColorPicker v-model:selection="bgColor"></ColorPicker>
  <ColorPicker supportsOpacity="false" v-model:selection="bgColor"></ColorPicker>
  <ColorPicker v-model:selection="bgColor">
    <Text>选择喜欢的颜色</Text>
  </ColorPicker>
</VStack> */
}

import SwiftCommon from './SwiftCommon.js'
// TODO: 在 output 中采用项目中差不多的文件结构
import SwiftColor from './SwiftColor.js'

// TODO: 使用 naive ui

export default {
  components: { SwiftCommon },
  name: 'ColorPicker',
  props: ['supportsOpacity', 'selection'],
  emits: ['update:selection'],
  setup(props, { emit }) {
    const supportsOpacity = props.supportsOpacity ?? true
    const picker = ref()

    const selection = computed(() => {
      return props.selection
    })

    watch(
      () => props.selection,
      (val) => {
        picker.value.value = val.toHex()
        picker.value.dispatchEvent(new Event('input', { bubbles: true }))
        // TODO: repeat when color change is occured by input
      },
    )

    const onColorRef = (ref) => {
      picker.value = ref

      if (props.selection) {
        picker.value.value = props.selection.toHex()
        picker.value.dispatchEvent(new Event('input', { bubbles: true }))
      }
    }

    const colorPickerClick = () => {
      Coloris({
        themeMode: 'polaroid',
        alpha: supportsOpacity,
        onChange: (color) => {
          const val = SwiftColor.fromHex(color)
          emit('update:selection', val)
        },
      })
    }

    return {
      selection,
      supportsOpacity,
      onColorRef,
      colorPickerClick,
    }
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <label class="color-picker built-in" :ref="(el) => setRef(el)">
        <span>
          <slot></slot>
        </span>
        <input
          class="color-picker-inner"
          type="text" data-coloris
          :ref="(el) => onColorRef(el)"
          @click="colorPickerClick"
        />
      </label>
    </template>
  </SwiftCommon>`,
}
