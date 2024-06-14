export default {
  name: "SwiftCommon",
  props: ['padding', 'buttonStyle', 'datePickerStyle'],
  setup(props) {
    const slotRef = ref();

    let paddingSize = 0;

    if (typeof props.padding === 'string' && props.padding.length == 0) {
      // default padding
      paddingSize = 16;
    } else if (typeof props.padding === 'string') {
      paddingSize = parseInt(props.padding);
    }

    // ======= styles =======
    if (typeof props.buttonStyle === 'string') {
      provide('buttonStyle', props.buttonStyle);
    } else {
      provide('buttonStyle', 'default');
    }

    if (typeof props.datePickerStyle === 'string') {
      provide('datePickerStyle', props.datePickerStyle);
    } else {
      provide('datePickerStyle', 'default');
    }
    // ======= styles =======

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
