import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "DisclosureGroup",
  props: ['title', 'isExpanded'],
  emits: ['update:isExpanded'],
  setup(props, { emit }) {

    const expandedNames = computed(() => {
      return props.isExpanded ? 'index' : undefined
    });

    const onUpdate = (expandedNames) => {
      if (expandedNames === undefined) {
        emit('update:isExpanded', false)
      }
      if (Array.isArray(expandedNames)) {
        emit('update:isExpanded', expandedNames.includes('index'))
      }
    }

    return {
      title: props.title,
      expandedNames,
      onUpdate
    }
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <n-collapse arrow-placement="right" class="disclosure-group built-in" :ref="(el) => setRef(el)" accordion :expanded-names="expandedNames" @update:expanded-names="onUpdate">
        <n-collapse-item :title="title" name="index">
          <slot></slot>
        </n-collapse-item>
      </n-collapse>
    </template>
  </SwiftCommon>`
};
