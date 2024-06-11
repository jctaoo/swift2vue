import SwiftUI

struct BasicButton: View {
    var body: some View {
        VStack {
            Button(action: signIn) {
                Text("登录")
            }
            Button("登录", action: signIn)
        }
    }

    func signIn() { print("已登录") }
}


struct ContextMenuButton: View {
    var body: some View {
        Form {
            TextField("用户名", text: $username)
        }
        .contextMenu {
            Button("剪切", action: cut)
            Button("复制", action: copy)
            Button("粘贴", action: paste)
        }
    }

    @State var username: String = ""
    @State var tmp: String = ""

    func cut() {
        tmp = username
        username = ""
    }
    func copy() { tmp = username }
    func paste() { username += tmp }
}

struct StylingButton: View {
    var body: some View {
        HStack {
            Button("登录", action: signIn)
            Button("注册", action: register)
        }
        .buttonStyle(PlainButtonStyle())
    }

    func signIn() { print("已登录") }
    func register() { print("注册新账号") }
}

struct Button_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            BasicButton()
            ContextMenuButton()
            StylingButton()
        }
    }
}
