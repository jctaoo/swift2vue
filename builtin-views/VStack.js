import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "VStack",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <div class="vstack built-in" :ref="(el) => setRef(el)"><slot></slot></div>
    </template>
  </SwiftCommon>`
};
