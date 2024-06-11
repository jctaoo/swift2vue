import SwiftUI

struct BasicButton: View {
    var body: some View {
        VStack {
            Button(action: signIn) {
                Text("登录1")
                    .padding()
            }
            Button("登录2", action: signIn)

            HStack {
                Button("登录3", action: signIn)
                Button("注册", action: register)
            }
            .buttonStyle(PlainButtonStyle())
            .padding()
        }
    }

    var count = 0

    func signIn() { print("已登录") }

    func register() { print("注册") }
}

struct Button_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            BasicButton()
        }
    }
}
