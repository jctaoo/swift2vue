import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "Text",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <span class="text built-in" :ref="(el) => setRef(el)"><slot></slot></span>
    </template>
  </SwiftCommon>`
};
