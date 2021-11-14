using System;

namespace CSharp.Products
{
    public class HtmlButton: IButton
    {
        public void Render()
        {
            Console.WriteLine("<Button>Test</Button>");
            OnClick();
        }

        public void OnClick()
        {
            Console.WriteLine("Click! The button says 'Hello World!'");
        }
    }
}