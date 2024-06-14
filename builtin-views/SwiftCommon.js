export default {
  name: "SwiftCommon",
  props: ['padding', 'buttonStyle'],
  setup(props) {
    const slotRef = ref();

    let paddingSize = 0;

    if (typeof props.padding === 'string' && props.padding.length == 0) {
      // default padding
      paddingSize = 16;
    } else if (typeof props.padding === 'string') {
      paddingSize = parseInt(props.padding);
    }

    if (typeof props.buttonStyle === 'string') {
      provide('buttonStyle', props.buttonStyle);
    } else {
      provide('buttonStyle', 'default');
    }

    // watch slotRef
    watch(slotRef, (slot) => {
      if (slot) {
        if (paddingSize > 0) {
          slot.style.padding = `${paddingSize}px`;
        }
      }
    });

    return {
      paddingSize,
      setRef: (el) => {
        slotRef.value = el;
      }
    }
  },
  template: `<slot name="child" :set-ref="setRef"></slot>`
};
