

def AAAA(m):
    int(m)
    print("AAAA")
    if m > -1:
        AAAA(m-1)
    else:
        input_center()



def End():
    print("Bye")
    return 0

def input_center():
    input_command = input("Please write command: ")
    command_list = ["h", "r", "slp", "e"]
    if input_command == command_list[0] :
        print("Command list: \n Help - h \n"
              " Recursia - r \n"
              " Sleep - slp \n"
              " End - e")
        input_center()
    elif input_command == command_list[1] :
        AAAA(input("Number for recursia: "))
    elif input_command == command_list[2] :
        print("Zzzzzz\n"
              "      zzzzzz\n"
              "             zzzzzzz\n")
        input_center()
    elif input_command == command_list[3] :
        End()
input_center()
