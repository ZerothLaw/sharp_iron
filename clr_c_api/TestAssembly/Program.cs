using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace TestAssembly
{
    class Seconds
    {
        readonly int Value_m;

        public Seconds(int value)
        {
            Value_m = value;
        }

        public int ToMilliseconds()
        {
            return Value_m * 1000;
        }
    }
    class Program
    {
        private readonly Random rand_m;
        Program()
        {
            rand_m = new Random(1337);
            Console.WriteLine(System.Reflection.Assembly.GetExecutingAssembly().GetName().ToString());
            Type t = typeof(System.Reflection.Assembly);
            Console.WriteLine(t.Assembly.GetName().ToString());
        }

        public void Run()
        {
            while (true)
            {
                Console.WriteLine("Run Loop");
                int iUsers = LoadUsers();

                ProcessUsers(iUsers);
                SendUsers(iUsers);

                Sleep(new Seconds(5));
            }
        }

        private int LoadUsers()
        {
            Console.WriteLine("Load Users");
            Sleep(new Seconds(1));
            return rand_m.Next(100);
        }

        private void ProcessUsers(int iUsers)
        {
            Console.WriteLine("Process Users: " + iUsers.ToString());
            Sleep(new Seconds(iUsers / 4));
        }

        private void SendUsers(int iUsers)
        {
            Console.WriteLine("Send Users: " + iUsers.ToString());
            Sleep(new Seconds((iUsers / 10) + 1));
        }

        private void Sleep(Seconds seconds)
        {
            System.Threading.Thread.Sleep(seconds.ToMilliseconds());
        }

        static void Main(string[] args)
        {
            Program prog = new Program();
            prog.Run();
        }
    }
}
