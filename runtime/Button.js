import SwiftCommon from "./SwiftCommon.js";
import { inject } from 'vue';

function getButtonStyleClass(buttonStyle) {
  if (buttonStyle.endsWith('()')) {
    return buttonStyle.slice(0, -2);
  }
  if (buttonStyle === "default") {
    return "DefaultButtonStyle";
  }
  return buttonStyle;
}

export default {
  components: { SwiftCommon },
  name: "Button",
  props: ['action'],
  setup(props) {
    const buttonStyle = inject('buttonStyle');

    const buttonStyleClass = buttonStyle ? getButtonStyleClass(buttonStyle) : '';

    const onClick = () => {
      if (props.action) {
        props.action()
      }
    }
    return {
      onClick,
      buttonStyleClass
    }
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <button class="button built-in" :class="buttonStyleClass" @click="onClick" :ref="(el) => setRef(el)"><slot></slot></button>
    </template>
  </SwiftCommon>`
};
