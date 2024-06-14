import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "HStack",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <div class="hstack built-in" :ref="(el) => setRef(el)"><slot></slot></div>
    </template>
  </SwiftCommon>`
};
