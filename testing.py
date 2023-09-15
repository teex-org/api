import requests
import os
import sys
import json
os.system("cd database; ./create.sh; cd ..")

class Request:
    baseurl = "http://localhost:8000/api/"

    def __init__(self):
        self.methode  = "GET"
        self.path     = ""
        self.data     = {}
        self.token    = None
        self.bearer   = None
        self.exp_code = 200
        self.exp_data = {}

    def send(self, message, exp_token= False):
        print("• " + message, end="")
        headers = {}
        if self.bearer != None:
            headers["Authorization"] = f"Bearer {self.bearer}"

        match self.methode :
            case "POST":
                headers["Content-Type"] = "application/json"
                response = requests.post(self.baseurl + self.path, json=self.data, headers=headers)
            case "PUT":
                headers["Content-Type"] = "application/json"
                response = requests.put(self.baseurl + self.path, json=self.data, headers=headers)
            case "GET":
                headers["Content-Type"] = "application/json"
                response = requests.get(self.baseurl + self.path, headers=headers)
            case "DELETE":
                headers["Content-Type"] = "application/json"
                response = requests.delete(self.baseurl + self.path, headers=headers)
            
    
        code = response.status_code
        
        rec_json = {}
        try:
            rec_json = response.json()
        except ValueError:
            pass

        if code != self.exp_code:
            print("  ❌")
            print(f'wrong code, receive {code} instead of {self.exp_code}')
            if txt:=response.text:
                print(f'response : {txt}')
            quit()
        
        if exp_token :
            try :
                token = rec_json["token"]
                del rec_json["token"]
                # print(f'token : {token}')
            except Exception as e:
                print("\nX " + message)
                print("can't acces token")
                print(response.text)
                return

        if self.exp_data != rec_json :
            
            print("  ❌")
            print(f'Status {code} ok but receive wrong data :')
            print(f'get     : { json.dumps(rec_json, indent=2)}')
            print(f'excpect : {json.dumps(self.exp_data, indent=2)}')
            quit()
        
        print("")
        if exp_token:
            return token

print("\n\n--- AUTH ---\n")
tokens = []
for user in ["user1","user2","user3"]:
    #region create user
    r         = Request()
    r.methode = "POST"
    r.path    = "auth/signin"
    r.data    = {
        "email"    : f"{user}@teex.com",
        "name"     : user,
        "password" : "123"
    }
    r.exp_code  = 200
    tokens.append(r.send(f"create {user}", True))
    # endregion
    #region check 
    r          = Request()
    r.methode  = "GET"
    r.path     = f"user/{user}@teex.com"
    r.exp_code = 200
    r.bearer   = tokens[-1]
    r.exp_data = {
        "email" : f"{user}@teex.com",
        "name"  : user
    }
    r.send("  └──> checked")
    #endregion
tkn_user1, tkn_user2, tkn_user3 = tokens

print("\n\n--- USER ---\n")

#region ! acces userX
r          = Request()
r.methode  = "GET"
r.path     = "user/userX@teex.com"
r.exp_code = 404
r.bearer   = tkn_user1
r.send("! acces userX")
# endregion

#region ! acces user1 with wrong token
r          = Request()
r.methode  = "GET"
r.path     = "user/user1@teex.com"
r.exp_code = 401
r.bearer   = "xxx"
r.send("! acces user1 with wrong token")
#endregion

#region delete user3
r          = Request()
r.methode  = "DELETE"
r.path     = "user/"
r.exp_code = 200
r.bearer   = tkn_user3
r.send("delete user1")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "user/user3@teex.com"
r.exp_code = 404
r.bearer   = tkn_user2
r.send("  └──> checked")
#endregion

print("\n\n--- PROJECT ---\n")

#region create project project1 
r          = Request()
r.methode  = "POST"
r.path     = "project/"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "name" : "project1"
}
r.exp_data = {
    "id" : 1
}
r.send("create project1")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/1"
r.exp_code = 200
r.exp_data = {
    "id": 1,
    "name": "project1",
    "lists": [],
    "users": [
        {
            "email": "user1@teex.com",
            "name": "user1",
            "right": 1
        }
    ]
}
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion

#region edit project1 
r          = Request()
r.methode  = "PUT"
r.path     = "project/1"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "name" : "PROJECT1"
}
r.send("edit project1 (user1)")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/1"
r.exp_code = 200
r.exp_data = {
    "id": 1,
    "name": "PROJECT1",
    "lists": [],
    "users": [
        {
            "email": "user1@teex.com",
            "name": "user1",
            "right": 1
        }
    ]
}
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion

#region access user1's projects 
r          = Request()
r.methode  = "GET"
r.path     = "project/mines"
r.exp_code = 200
r.exp_data = {
    "projects": [{
        "id" : 1,
        "name" : "PROJECT1"
    }]
}
r.bearer   = tkn_user1
r.send("access user1's projects")
#endregion

#region ! delete project1 from user2
r          = Request()
r.methode  = "DELETE"
r.path     = "project/1"
r.exp_code = 403
r.bearer   = tkn_user2
r.send("! delete project1 from user2")
#endregion

#region delete project1 from user1
r          = Request()
r.methode  = "DELETE"
r.path     = "project/1"
r.exp_code = 200
r.bearer   = tkn_user1
r.send("delete project1 from user1")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/1"
r.exp_code = 403
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion

#region () create project project1 
r          = Request()
r.methode  = "POST"
r.path     = "project/"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "name" : "project1"
}
r.exp_data = {
    "id" : 2
}
r.send("() create project project1 (user1)")
#endregion


print("\n\n--- LIST ---\n")

#region create list in project1 (user2) 
r          = Request()
r.methode  = "POST"
r.path     = "list/"
r.exp_code = 403
r.bearer   = tkn_user2
r.data     = {
    "id_project" : 2,
    "name" : "list1",
}

r.send("! create list1 (user2)")
#endregion

#region create list in project1 (user1) 
r          = Request()
r.methode  = "POST"
r.path     = "list/"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "id_project" : 2,
    "name" : "list1",
}
r.exp_data     = {
    "id" : 1,
}
r.send("create list1 (user1)")
#endregion

#region ! delete list1 in project1 (user2) 
r          = Request()
r.methode  = "DELETE"
r.path     = "list/1"
r.exp_code = 403
r.bearer   = tkn_user2


r.send("! delete list1 (user2)")
#endregion

#region create list in project1 (user1) 
r          = Request()
r.methode  = "DELETE"
r.path     = "list/1"
r.exp_code = 200
r.bearer   = tkn_user1
r.send("delete list1 (user1)")
#endregion

#region () create list in project1 (user1) 
r          = Request()
r.methode  = "POST"
r.path     = "list/"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "id_project" : 2,
    "name" : "list1",
}
r.exp_data     = {
    "id" : 2,
}
r.send("() create list1 (user1)")
#endregion

#region ! edit list1 (user2) 
r          = Request()
r.methode  = "PUT"
r.path     = "list/2"
r.exp_code = 403
r.bearer   = tkn_user2
r.data     = {
    "name" : "LIST1",
}
r.send("! edit list1 (user2)")
#endregion

#region edit list1 (user1) 
r          = Request()
r.methode  = "PUT"
r.path     = "list/2"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "name" : "LIST1",
}
r.send("edit list1 (user1)")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/2"
r.exp_code = 200
r.exp_data = {
    "id": 2,
    "name": "project1",
    "lists": [
        {
            "id"    : 2,
            "name"  : "LIST1",
            "tasks" : []
        }
    ],
    "users": [
        {
            "email": "user1@teex.com",
            "name": "user1",
            "right": 1
        }
    ]
}
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion

print("\n\n--- TASK ---\n")
#region ! create task1 (user2)
r          = Request()
r.methode  = "POST"
r.path     = "task/"
r.exp_code = 403
r.bearer   = tkn_user2
r.data     = {
    "name"     : "task1",
    "priority" : 0,
    "tag"      : 1,
    "id_list"  : 2
}
r.send("! create task1 (user2)")
#endregion

#region create task1 (user1)
r          = Request()
r.methode  = "POST"
r.path     = "task/"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    "name"     : "task1",
    "priority" : 0,
    "tag"      : 1,
    "id_list"  : 2
}
r.exp_data     = {
    "id" : 1,
}
r.send("create task1 (user1)")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/2"
r.exp_code = 200
r.exp_data ={
  "id": 2,
  "name": "project1",
  "lists": [
    {
      "id": 2,
      "name": "LIST1",
      "tasks": [
        {
          "id"             : 1,
          "id_list"        : 2,
          "name"           : "task1",
          "descr"          : "",
          "tag"            : 1,
          "priority"       : 0,
          "state"          : 0,
          "id_last_editor" : "user1@teex.com"
        }
      ]
    }
  ],
  "users": [
    {
      "email": "user1@teex.com",
      "name": "user1",
      "right": 1
    }
  ]
}
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion


#region ! edit task1 (user2)
r          = Request()
r.methode  = "PUT"
r.path     = "task/1"
r.exp_code = 403
r.bearer   = tkn_user2
r.data     = {
    "name"     : "TASK1",
    "descr"    : "Lorem ipsum",
    "priority" : 1,
    "tag"      : 5,
}
r.send("! edit task1 (user2)")
#endregion

#region edit task1 (user1)
r          = Request()
r.methode  = "PUT"
r.path     = "task/1"
r.exp_code = 200
r.bearer   = tkn_user1
r.data     = {
    # "name"     : "task1",
    "descr"    : "Lorem ipsum",
    # "priority" : 0,
    "tag"      : 5,
}
r.send("edit task1 (user1)")
#endregion

#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/2"
r.exp_code = 200
r.exp_data = {
  "id": 2,
  "name": "project1",
  "lists": [
    {
      "id": 2,
      "name": "LIST1",
      "tasks": [
        {
          "id": 1,
          "id_list": 2,
          "name": "task1",
          "descr": "Lorem ipsum",
          "tag": 5,
          "priority": 0,
          "state": 0,
          "id_last_editor": "user1@teex.com"
        }
      ]
    }
  ],
  "users": [
    {
      "email": "user1@teex.com",
      "name": "user1",
      "right": 1
    }
  ]
}
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion


#region ! edit task1 (user2)
r          = Request()
r.methode  = "PUT"
r.path     = "task/state/1"
r.exp_code = 403
r.bearer   = tkn_user2
r.send("! edit state task1 (user2)")
#endregion

#region edit state task1 (user1)
r          = Request()
r.methode  = "PUT"
r.path     = "task/state/1"
r.exp_code = 200
r.bearer   = tkn_user1
r.send("edit state task1 (user1)")
#endregion


#region check 
r          = Request()
r.methode  = "GET"
r.path     = "project/2"
r.exp_code = 200
r.exp_data = {
  "id": 2,
  "name": "project1",
  "lists": [
    {
      "id": 2,
      "name": "LIST1",
      "tasks": [
        {
          "id": 1,
          "id_list": 2,
          "name": "task1",
          "descr": "Lorem ipsum",
          "tag": 5,
          "priority": 0,
          "state": 1,
          "id_last_editor": "user1@teex.com"
        }
      ]
    }
  ],
  "users": [
    {
      "email": "user1@teex.com",
      "name": "user1",
      "right": 1
    }
  ]
}
r.bearer   = tkn_user1
r.send("  └──> checked")
#endregion

