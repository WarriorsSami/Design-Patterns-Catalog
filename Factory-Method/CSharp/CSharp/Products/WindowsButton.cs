using System;

namespace CSharp.Products
{
    public class WindowsButton: IButton
    {
        public void Render()
        {
            Console.WriteLine("Render Windows Button!");
            OnClick();
        }

        public void OnClick()
        {
            Console.WriteLine("Windows Button Clicked!");
        }
    }
}