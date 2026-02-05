using UnityEngine;

public class SecureLockdown : MonoBehaviour
{
    public string BackendUrl = "http://localhost:8080";

    void Start()
    {
        Debug.Log("SecureLockdown initialized with backend " + BackendUrl);
    }
}
