---

Action= Enum{"merge", "rebase", "commit"} | Int{0-2}

Job= {
  "before_script"= [Text]
  "script"= [Text]
  "after_script"= [Text]
  "action"= Action{"undo"}
}

~["<job>"]= Job

!"stages"= [Text] | [{"name"= Text, "unique"= Bool}]

Remote= {
  "name"= Text
  ~"url"= Text :"localhost"
  !"port"= Int{0-60000} :8080
  "username"= Text
  "http-passwd"= Text
}

"remotes"= [Remote] | [Remote{"port":29418, ^"http-passwd", !"proxy"= Text}] | Text{a-z} | Null | Empty

...