VStack {
  Text("Button Demo")
  HStack {
    Button("Text Button") {
      print("Text Button Clicked")
    }

    Button("Background Button") {
      print("Background Button Clicked")
    }.background(Color.blue)

    Button("Green Button") {
      print("Green Button Clicked")
    }.background(Color.green)
  }
  HStack {
    Button("Red Text Btn") {
      print("Red Text Btn Clicked")
    }.foregroundStyle(.red)

    Button("Blue Text Yellow Btn") {
      print("Blue Text Yellow Btn Clicked")
    }.foregroundStyle(.blue).background(Color.yellow)
  }
}