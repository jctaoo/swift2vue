import SwiftUI

struct BasicButton: View {
    var body: some View {
        VStack {
            Button(action: signIn) {
                Text("登录")
                    .padding()
            }
            Button("登录", action: signIn)

            HStack {
                Button("登录", action: signIn)
                Button("注册", action: register)
            }
            .buttonStyle(PlainButtonStyle())
            .padding()
            .background(.blue)
        }
    }

    var count = 0

    func signIn() { print("已登录") }

    func register() { print("注册") }
}