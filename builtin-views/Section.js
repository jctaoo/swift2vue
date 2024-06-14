import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "Section",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <div class="section built-in" :ref="(el) => setRef(el)"><slot></slot></div>
    </template>
  </SwiftCommon>`
};
