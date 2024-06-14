import SwiftCommon from './SwiftCommon.js'
// TODO: 在 output 中采用项目中差不多的文件结构
import SwiftDate from './SwiftDate.js'
import DatePickerComponents from './DatePickerComponents.js'

function getDatePickerStyle(style) {
  if (style.endsWith('()')) {
    return style.slice(0, -2);
  }
  if (style === "default") {
    return "DefaultDatePickerStyle";
  }
  return style;
}

export default {
  components: { SwiftCommon },
  name: 'DatePicker',
  props: ['displayedComponents', 'selection', 'datePickerStyle'],
  emits: ['update:selection'],
  setup(props, { emit }) {
    const datePickerStyle = props.datePickerStyle ?? inject('datePickerStyle', 'default')
    const datePickerStyleFixed = getDatePickerStyle(datePickerStyle);

    const usePanel = computed(() => {
      return datePickerStyleFixed === 'GraphicalDatePickerStyle'
    });

    const displayedComponents = props.displayedComponents ?? [DatePickerComponents.date];
    const includeTime = displayedComponents.map(i => i.rawValue).includes(DatePickerComponents.hourAndMinute.rawValue);
    const pickerType = computed(() => {
      if (includeTime) {
        return 'datetime';
      }
      return 'date';
    });

    const timestamp = computed({
      get: () => {
        return props.selection.date.getTime()
      },
      set: (val) => {
        const newDate = SwiftDate.fromTimestamp(val)
        emit('update:selection', newDate)
      },
    })

    return {
      timestamp,
      pickerType,
      usePanel,
    }
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <label class="date-picker built-in" :ref="(el) => setRef(el)">
        <span v-if="!usePanel">
          <slot></slot>
        </span>
        <n-date-picker v-model:value="timestamp" :type="pickerType" size="small" :panel="usePanel" />
      </label>
    </template>
  </SwiftCommon>`,
}
